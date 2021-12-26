// [[file:../xtb.note::fb6f72a1][fb6f72a1]]
use super::*;

use std::ptr::null;
// fb6f72a1 ends here

// [[file:../xtb.note::8cd490ab][8cd490ab]]
/// XTB Calculation environment
pub struct XtbEnvironment {
    /// XTB Calculation environment
    env: xtb_TEnvironment,
}

impl XtbEnvironment {
    /// Create new xtb calculation environment object
    pub fn new() -> Self {
        unsafe { assert_eq!(XTB_API_VERSION, xtb_getAPIVersion() as u32) };

        Self {
            env: unsafe { xtb_newEnvironment() },
        }
    }

    /// Check current status of calculation environment.
    pub fn check_error(&self) -> Result<()> {
        let ret = unsafe { xtb_checkEnvironment(self.env) };
        let mut err_msg = vec![];
        if ret != 0 {
            // Show and empty error stack
            unsafe { xtb_showEnvironment(self.env, err_msg.as_mut_ptr()) };
            // TODO: show formated error message
            bail!("Error occured in the API with code {}!", ret);
        }
        Ok(())
    }

    /// Set verbosity of calculation output.
    fn set_verbosity(&mut self, verbosity: u32) -> Result<()> {
        unsafe {
            xtb_setVerbosity(self.env, verbosity as i32);
        }
        self.check_error()?;
        Ok(())
    }

    /// Set verbosity of calculation output to be verbose
    pub fn set_output_verbose(&mut self) -> Result<()> {
        self.set_verbosity(XTB_VERBOSITY_FULL)
    }

    /// Set verbosity of calculation output to be minimal
    pub fn set_output_minimal(&mut self) -> Result<()> {
        self.set_verbosity(XTB_VERBOSITY_MINIMAL)
    }

    /// Set verbosity of calculation output to be muted
    pub fn set_output_muted(&mut self) -> Result<()> {
        self.set_verbosity(XTB_VERBOSITY_MUTED)
    }
}
// 8cd490ab ends here

// [[file:../xtb.note::3bbaae4e][3bbaae4e]]
pub struct XtbMolecule {
    mol: xtb_TMolecule,
}

impl XtbMolecule {
    fn new(mol: xtb_TMolecule) -> Self {
        Self { mol }
    }

    /// Create new molecular structure data (quantities in Bohr).
    fn create(env: &mut XtbEnvironment, attyp: &[i32], coord: &[f64], charge: f64, uhf: i32) -> Result<Self> {
        let mol = unsafe {
            let natoms = attyp.len() as i32;
            let env = env.env;
            let attyp = attyp.as_ptr();
            let coord = coord.as_ptr();
            xtb_newMolecule(env, &natoms, attyp, coord, &charge, &uhf, null(), null())
        };
        env.check_error()?;
        let mol = Self { mol };

        Ok(mol)
    }
}
// 3bbaae4e ends here

// [[file:../xtb.note::e737b33d][e737b33d]]
/// XTB single point calculator
pub struct XtbCalculator {
    calc: xtb_TCalculator,
}

impl XtbCalculator {
    /// Create new calculator object
    pub fn new() -> Self {
        Self {
            calc: unsafe { xtb_newCalculator() },
        }
    }

    // Set parametrization of GFN-xTB method. GFN2-xTB is the default
    // parametrization. Also available are GFN1-xTB, GFN0-xTB.
    fn load_gfn(&mut self, mol: &XtbMolecule, env: &mut XtbEnvironment, n: usize) -> Result<()> {
        unsafe {
            let calc = self.calc;
            let mol = mol.mol;
            let env = env.env;
            match n {
                0 => xtb_loadGFN0xTB(env, mol, calc, std::ptr::null_mut()),
                1 => xtb_loadGFN1xTB(env, mol, calc, std::ptr::null_mut()),
                2 => xtb_loadGFN2xTB(env, mol, calc, std::ptr::null_mut()),
                _ => unimplemented!(),
            }
        }
        env.check_error()?;
        Ok(())
    }

    pub fn single_point(&mut self, mol: &XtbMolecule, env: &mut XtbEnvironment) -> Result<XtbResults> {
        let mut res = XtbResults::new();
        unsafe {
            let calc = self.calc;
            let mol = mol.mol;
            let res = res.res;
            let env = env.env;
            xtb_singlepoint(env, mol, calc, res);
        }
        env.check_error()?;
        Ok(res)
    }
}
// e737b33d ends here

// [[file:../xtb.note::1e3dd6ef][1e3dd6ef]]
/// XTB singlepoint results object
pub struct XtbResults {
    res: xtb_TResults,
}

impl XtbResults {
    /// Create new singlepoint results object
    fn new() -> Self {
        Self {
            res: unsafe { xtb_newResults() },
        }
    }

    /// Get singlepoint energy in Hartree
    pub fn get_energy(&self, env: &XtbEnvironment) -> Result<f64> {
        let mut energy = std::f64::NAN;
        unsafe {
            xtb_getEnergy(env.env, self.res, &mut energy);
        }
        env.check_error()?;
        Ok(energy)
    }

    /// Get dipole in e Bohr
    pub fn get_dipole(&self, env: &XtbEnvironment) -> Result<[f64; 3]> {
        let mut dipole = [std::f64::NAN; 3];
        unsafe {
            xtb_getDipole(env.env, self.res, dipole.as_mut_ptr());
        }
        env.check_error()?;
        Ok(dipole)
    }

    /// Get gradient in Hartree / Bohr
    pub fn get_gradient(&self, env: &XtbEnvironment, gradient: &mut [f64]) -> Result<()> {
        unsafe {
            xtb_getDipole(env.env, self.res, gradient.as_mut_ptr());
        }
        env.check_error()?;
        Ok(())
    }
}
// 1e3dd6ef ends here

// [[file:../xtb.note::7d8b4594][7d8b4594]]
macro_rules! impl_xtb_drop {
    ($obj:ident, $xtb_del:ident, $res:ident) => {
        impl Drop for $obj {
            fn drop(&mut self) {
                if !self.$res.is_null() {
                    unsafe { $xtb_del(&mut self.$res) }
                }
                assert!(self.$res.is_null());
            }
        }
    };
}

impl_xtb_drop!(XtbEnvironment, xtb_delEnvironment, env);
impl_xtb_drop!(XtbMolecule, xtb_delMolecule, mol);
impl_xtb_drop!(XtbResults, xtb_delResults, res);
impl_xtb_drop!(XtbCalculator, xtb_delCalculator, calc);
// 7d8b4594 ends here

// [[file:../xtb.note::1c066e9f][1c066e9f]]
#[test]
fn test_xtb_raw_api() -> Result<()> {
    let coord = test::ATOM_COORDS;
    let attyp = [6, 6, 6, 1, 1, 1, 1];

    let mut env = XtbEnvironment::new();
    let mol = XtbMolecule::create(&mut env, &attyp, &coord, 0.0, 0)?;
    let mut calc = XtbCalculator::new();
    calc.load_gfn(&mol, &mut env, 2)?;
    let res = calc.single_point(&mol, &mut env)?;
    let energy = res.get_energy(&env)?;
    let dipole = res.get_dipole(&env)?;
    assert!((energy + 8.3824793849585).abs() < 1.0e-9);
    assert!((dipole[2] + 0.298279305689518).abs() < 1.0e-6);

    calc.load_gfn(&mol, &mut env, 1)?;
    let res = calc.single_point(&mol, &mut env)?;
    let energy = res.get_energy(&env)?;
    assert!((energy + 8.424757953815186).abs() < 1.0e-9);

    Ok(())
}
// 1c066e9f ends here

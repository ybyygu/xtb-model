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
    fn set_verbosity(&self, verbosity: u32) -> Result<()> {
        unsafe {
            xtb_setVerbosity(self.env, verbosity as i32);
        }
        self.check_error()?;
        Ok(())
    }

    /// Set verbosity of calculation output to be verbose
    pub fn set_output_verbose(&self) -> Result<()> {
        self.set_verbosity(XTB_VERBOSITY_FULL)
    }

    /// Set verbosity of calculation output to be minimal
    pub fn set_output_minimal(&self) -> Result<()> {
        self.set_verbosity(XTB_VERBOSITY_MINIMAL)
    }

    /// Set verbosity of calculation output to be muted
    pub fn set_output_muted(&self) -> Result<()> {
        self.set_verbosity(XTB_VERBOSITY_MUTED)
    }
}
// 8cd490ab ends here

// [[file:../xtb.note::3bbaae4e][3bbaae4e]]
/// Molecular structure data
pub struct XtbMolecule {
    mol: xtb_TMolecule,
}

impl XtbMolecule {
    /// Create new molecular structure data (quantities in Bohr).
    pub fn create(env: &XtbEnvironment, attyp: &[i32], coord: &[f64], charge: f64, uhf: i32) -> Result<Self> {
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

    /// Update coordinates and lattice parameters (quantities in Bohr)
    pub fn update(&self, env: &XtbEnvironment, coord: &[f64], lattice: Option<[f64; 9]>) -> Result<()> {
        unsafe {
            let env = env.env;
            let mol = self.mol;
            let coord = coord.as_ptr();
            if let Some(lat) = lattice {
                xtb_updateMolecule(env, mol, coord, lat.as_ptr());
            } else {
                xtb_updateMolecule(env, mol, coord, null());
            }
        }
        env.check_error()?;

        Ok(())
    }
}
// 3bbaae4e ends here

// [[file:../xtb.note::e737b33d][e737b33d]]
/// Possible parametrisations for the Calculator.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum XtbMethod {
    /// GFN2-xTB
    GFN2xTB,
    /// GFN1-xTB
    GFN1xTB,
    /// GFN0-xTB
    GFN0xTB,
    /// GFN0-FF
    GFNFF,
}

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

    /// Set parametrization of GFN-xTB method.
    pub fn set_method(&self, mol: &XtbMolecule, env: &XtbEnvironment, method: XtbMethod) -> Result<()> {
        unsafe {
            let calc = self.calc;
            let mol = mol.mol;
            let env = env.env;
            match method {
                XtbMethod::GFNFF => xtb_loadGFNFF(env, mol, calc, std::ptr::null_mut()),
                XtbMethod::GFN0xTB => xtb_loadGFN0xTB(env, mol, calc, std::ptr::null_mut()),
                XtbMethod::GFN1xTB => xtb_loadGFN1xTB(env, mol, calc, std::ptr::null_mut()),
                XtbMethod::GFN2xTB => xtb_loadGFN2xTB(env, mol, calc, std::ptr::null_mut()),
                _ => unimplemented!(),
            }
        }
        env.check_error()?;
        Ok(())
    }

    /// Set maximum number of iterations for self-consistent TB calculators.
    pub fn set_max_iterations(&self, env: &XtbEnvironment, n: usize) {
        unsafe {
            xtb_setMaxIter(env.env, self.calc, n as i32);
        }
    }

    /// Set electronic temperature for level filling in tight binding calculators in K
    pub fn set_electronic_temperature(&self, env: &XtbEnvironment, temp: f64) {
        unsafe {
            xtb_setElectronicTemp(env.env, self.calc, temp);
        }
    }

    /// Set numerical accuracy of calculator in the range of 1000 to 0.0001
    pub fn set_accuracy(&self, env: &XtbEnvironment, acc: f64) {
        unsafe {
            xtb_setAccuracy(env.env, self.calc, acc);
        }
    }

    /// Perform singlepoint calculation.
    pub fn single_point(&self, mol: &XtbMolecule, env: &XtbEnvironment) -> Result<XtbResults> {
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

    /// Query singlepoint results object for bond orders
    pub fn get_bond_orders(&self, env: &XtbEnvironment, bond_orders: &mut [f64]) -> Result<()> {
        unsafe {
            xtb_getBondOrders(env.env, self.res, bond_orders.as_mut_ptr());
        }
        env.check_error()?;
        Ok(())
    }

    /// Query singlepoint results object for partial charges in e
    pub fn get_charges(&self, env: &XtbEnvironment, charges: &mut [f64]) -> Result<()> {
        unsafe {
            xtb_getCharges(env.env, self.res, charges.as_mut_ptr());
        }
        env.check_error()?;
        Ok(())
    }

    /// Query singlepoint results object for virial in Hartree
    pub fn get_virial(&self, env: &XtbEnvironment, virial: &mut [f64]) -> Result<()> {
        unsafe {
            xtb_getVirial(env.env, self.res, virial.as_mut_ptr());
        }
        env.check_error()?;
        Ok(())
    }

    /// Query singlepoint results object for the number of basis functions
    pub fn get_nao(&self, env: &XtbEnvironment) -> Result<usize> {
        let mut nao = 0;
        unsafe {
            xtb_getNao(env.env, self.res, &mut nao);
        }
        env.check_error()?;
        Ok(nao as usize)
    }

    /// Query singlepoint results object for orbital energies in Hartree [nao]
    pub fn get_orbital_eigenvalues(&self, env: &XtbEnvironment, emo: &mut [f64]) -> Result<()> {
        unsafe {
            xtb_getOrbitalEigenvalues(env.env, self.res, emo.as_mut_ptr());
        }
        env.check_error()?;
        Ok(())
    }

    /// Query singlepoint results object for occupation numbers [nao]
    pub fn get_orbital_occupations(&self, env: &XtbEnvironment, focc: &mut [f64]) -> Result<()> {
        unsafe {
            xtb_getOrbitalOccupations(env.env, self.res, focc.as_mut_ptr());
        }
        env.check_error()?;
        Ok(())
    }

    /// Query singlepoint results object for orbital coefficients [nao][nao]
    pub fn get_orbital_coefficients(&self, env: &XtbEnvironment, forb: &mut [f64]) -> Result<()> {
        unsafe {
            xtb_getOrbitalCoefficients(env.env, self.res, forb.as_mut_ptr());
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

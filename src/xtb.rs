// [[file:../xtb.note::a89844af][a89844af]]
//! high level wrapper for xTB
// a89844af ends here

// [[file:../xtb.note::a7b88800][a7b88800]]
use super::*;

use libxtb::*;
// a7b88800 ends here

// [[file:../xtb.note::392dc74e][392dc74e]]
/// Possible parameters for XTB calculation.
#[derive(Clone, Debug)]
pub struct XtbParameters {
    uhf: usize,
    charge: f64,
    verbosity: XtbOutputVerbosity,
    max_iterations: usize,
    electronic_temperature: f64,
    method: XtbMethod,
    // TODO: solvent
}

#[derive(Clone, Debug)]
pub enum XtbOutputVerbosity {
    Muted,
    Minimal,
    Verbose,
}

impl From<&str> for XtbMethod {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GFN2XTB" | "GFN2-XTB" => XtbMethod::GFN2xTB,
            "GFN1XTB" | "GFN1-XTB" => XtbMethod::GFN1xTB,
            "GFN0XTB" | "GFN0-XTB" => XtbMethod::GFN0xTB,
            "GFNFF" | "GFN-FF" => XtbMethod::GFNFF,
            _ => panic!("invalid xTB method: {}", s),
        }
    }
}

impl Default for XtbParameters {
    fn default() -> Self {
        Self {
            uhf: 0,
            charge: 0.0,
            verbosity: XtbOutputVerbosity::Muted,
            max_iterations: 250,
            electronic_temperature: 300.0,
            method: XtbMethod::GFN2xTB,
        }
    }
}

impl XtbParameters {
    /// Set system charge `charge`.
    pub fn charge(&mut self, charge: f64) -> &mut Self {
        self.charge = charge;
        self
    }

    /// Set `n` unpaired electrons.
    pub fn unpaired_electrons(&mut self, n: usize) -> &mut Self {
        self.uhf = n;
        self
    }

    /// Set electronic temperature for level filling in tight binding calculators in K.
    pub fn electronic_temperature(&mut self, t: f64) -> &mut Self {
        assert!(t.is_sign_positive(), "invalid temperature {:?}", t);
        self.electronic_temperature = t;
        self
    }

    /// Set maximum number of iterations for self-consistent TB calculators.
    pub fn max_iterations(&mut self, n: usize) -> &mut Self {
        self.max_iterations = n;
        self
    }

    /// Set calculation output to be verbose.
    pub fn output_verbose(&mut self) -> &mut Self {
        self.verbosity = XtbOutputVerbosity::Verbose;
        self
    }

    /// Set calculation output to be minmal.
    pub fn output_minimal(&mut self) -> &mut Self {
        self.verbosity = XtbOutputVerbosity::Minimal;
        self
    }

    /// Set calculation output to be muted.
    pub fn output_muted(&mut self) -> &mut Self {
        self.verbosity = XtbOutputVerbosity::Muted;
        self
    }

    /// Set xTB class of method
    pub fn method<M: Into<XtbMethod>>(&mut self, method: M) -> &mut Self {
        self.method = method.into();
        self
    }
}
// 392dc74e ends here

// [[file:../xtb.note::bcd483ad][bcd483ad]]
/// High level abstraction for XTB evaluation of energy and gradient
pub struct XtbModel {
    params: XtbParameters,
    atom_types: Vec<i32>,
    coord: Vec<f64>,

    env: XtbEnvironment,
    mol: XtbMolecule,
    calc: XtbCalculator,

    // calculated results
    dipole: Option<[f64; 3]>,
}

impl XtbModel {
    /// Construct new XtbModel for atoms specified with atomic numbers in
    /// `atom_types`.
    pub fn create<P: Into<Option<XtbParameters>>>(atom_types: &[usize], coord: &[f64], params: P) -> Result<Self> {
        assert_eq!(
            atom_types.len() * 3,
            coord.len(),
            "Dimension missmatch between numbers and positions"
        );

        let env = XtbEnvironment::new();
        let atom_types: Vec<_> = atom_types.iter().map(|&x| x as i32).collect();
        let params = params.into().unwrap_or_default();
        match params.verbosity {
            XtbOutputVerbosity::Verbose => env.set_output_verbose()?,
            XtbOutputVerbosity::Muted => env.set_output_muted()?,
            XtbOutputVerbosity::Minimal => env.set_output_minimal()?,
        }

        let uhf = params.uhf as i32;
        let charge = params.charge;
        let mol = XtbMolecule::create(&env, &atom_types, coord, charge, uhf)?;
        let mut calc = XtbCalculator::new();
        calc.set_method(&mol, &env, params.method)?;
        let xtb = Self {
            coord: coord.to_vec(),
            dipole: None,
            mol,
            calc,

            params,
            atom_types,
            env,
        };

        Ok(xtb)
    }

    /// Update coordinates and lattice parameters (quantities in Bohr).
    pub fn update_structure(&mut self, positions: &[f64], lattice: Option<[f64; 9]>) -> Result<()> {
        assert_eq!(positions.len(), self.coord.len());

        self.coord.clone_from_slice(positions);
        if let Some(lat) = lattice {
            unimplemented!();
        }

        Ok(())
    }

    /// Call XTB for evaluation of energy and gradient. coord in bohr.
    pub fn calculate_energy_and_gradient(&mut self, gradient: &mut [f64]) -> Result<f64> {
        let env = &self.env;
        let mol = &self.mol;

        // FIXME: lattice
        mol.update(env, &self.coord, None)?;
        self.calc.set_method(mol, env, self.params.method)?;
        self.calc.set_accuracy(env, 1.0);
        self.calc.set_electronic_temperature(env, self.params.electronic_temperature);
        self.calc.set_max_iterations(env, self.params.max_iterations);
        let res = self.calc.single_point(mol, env)?;
        let energy = res.get_energy(env)?;
        res.get_gradient(env, gradient)?;
        self.dipole = res.get_dipole(env)?.into();

        Ok(energy)
    }

    /// Return last evaluated dipole moment. Return None if not calculated yet.
    pub fn get_dipole(&self) -> Option<[f64; 3]> {
        self.dipole
    }
}
// bcd483ad ends here

// [[file:../xtb.note::2398beeb][2398beeb]]
#[test]
fn test_xtb_method_into() {
    let m: XtbMethod= "GFN0xTB".into();
    assert_eq!(m, XtbMethod::GFN0xTB);
    let m: XtbMethod= "GFN0-xTB".into();
    assert_eq!(m, XtbMethod::GFN0xTB);
    let m: XtbMethod= "gfn0-xtb".into();
    assert_eq!(m, XtbMethod::GFN0xTB);

    let m: XtbMethod= "GFN1xTB".into();
    assert_eq!(m, XtbMethod::GFN1xTB);
    let m: XtbMethod= "GFN2xTB".into();
    assert_eq!(m, XtbMethod::GFN2xTB);
    let m: XtbMethod= "GFNFF".into();
    assert_eq!(m, XtbMethod::GFNFF);
    let m: XtbMethod= "GFN-FF".into();
    assert_eq!(m, XtbMethod::GFNFF);
}

#[should_panic]
fn test_xtb_method_into_panic() {
    let m: XtbMethod= "gfn-xtb".into();
}
// 2398beeb ends here

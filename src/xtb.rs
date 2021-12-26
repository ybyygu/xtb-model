// [[file:../xtb.note::a7b88800][a7b88800]]
use super::*;

use libxtb::*;
// a7b88800 ends here

// [[file:../xtb.note::bcd483ad][bcd483ad]]
/// Possible parameters for XTB calculation.
#[derive(Clone, Debug)]
pub struct XtbParameters {
    uhf: usize,
    charge: f64,
    verbosity: XtbOutputVerbosity,
    max_iterations: usize,
    electronic_temperature: f64,
    // TODO: solvent
    // method:
    // accuracy:
}

#[derive(Clone, Debug)]
pub enum XtbOutputVerbosity {
    Muted,
    Minimal,
    Verbose,
}

impl Default for XtbParameters {
    fn default() -> Self {
        Self {
            uhf: 0,
            charge: 0.0,
            verbosity: XtbOutputVerbosity::Muted,
            // TODO
            max_iterations: 250,
            electronic_temperature: 300.0,
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
}

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
        let xtb = Self {
            coord: coord.to_vec(),
            mol: XtbMolecule::create(&env, &atom_types, coord, charge, uhf)?,
            calc: XtbCalculator::new(),
            dipole: None,

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
        self.calc.load_gfn(mol, env, 2)?;
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

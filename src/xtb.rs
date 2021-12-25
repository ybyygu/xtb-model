// [[file:../xtb.note::a7b88800][a7b88800]]
use super::*;

use anyhow::*;
// a7b88800 ends here

// [[file:../xtb.note::bcd483ad][bcd483ad]]
/// Call XTB for evaluation of energy and gradient
pub struct XtbModel {
    atom_types: Vec<i32>,
    charge: f64,
    uhf: usize,
    dipole: Option<[f64; 3]>,
}

impl XtbModel {
    /// Construct new XtbModel for atoms specified with atomic numbers in
    /// `atom_types`.
    pub fn new(atom_types: &[usize]) -> Self {
        Self {
            atom_types: atom_types.iter().map(|&x| x as i32).collect(),
            charge: 0.0,
            uhf: 0,
            dipole: None,
        }
    }

    /// With system charge `charge`.
    pub fn with_charge(mut self, charge: f64) -> Self {
        self.charge = charge;
        self
    }

    /// With `n` unpaired electrons.
    pub fn with_unpaired_electrons(mut self, n: usize) -> Self {
        self.uhf = n;
        self
    }

    /// Call XTB for evaluation of energy and gradient
    pub fn calculate_energy_and_gradient(&mut self, coord: &[f64], gradient: &mut [f64]) -> Result<f64> {
        let n = self.atom_types.len();
        assert_eq!(coord.len(), gradient.len(), "invalid array sizes");
        assert_eq!(coord.len(), n * 3, "not a flatten cartesian coord array");

        let mut energy = 0.0;
        let mut dipole = [0.0; 3];
        let charge = self.charge;
        let natoms = n as i32;
        let uhf = self.uhf.try_into().expect("uhf too large");

        let ret = unsafe {
            let coord = coord.as_ptr();
            let attyp = self.atom_types.as_ptr();
            let gradient = gradient.as_mut_ptr();
            let dipole = dipole.as_mut_ptr();
            xtb_calculate_energy_and_gradient(coord, attyp, natoms, charge, uhf, &mut energy, gradient, dipole)
        };
        if ret != 0 {
            bail!("xtb error code: {}", ret);
        }
        self.dipole = dipole.into();

        Ok(energy)
    }

    /// Return last evaluated dipole moment. Return None if not calculated yet.
    pub fn get_dipole(&self) -> Option<[f64; 3]> {
        self.dipole
    }
}
// bcd483ad ends here

// [[file:../xtb.note::1e609bf6][1e609bf6]]
#[test]
fn test_xtb_energy_and_gradient() -> Result<()> {
    let coord = [
        0.00000000000000,
        0.00000000000000,
        -1.79755622305860,
        0.00000000000000,
        0.00000000000000,
        0.95338756106749,
        0.00000000000000,
        0.00000000000000,
        3.22281255790261,
        -0.96412815539807,
        -1.66991895015711,
        -2.53624948351102,
        -0.96412815539807,
        1.66991895015711,
        -2.53624948351102,
        1.92825631079613,
        0.00000000000000,
        -2.53624948351102,
        0.00000000000000,
        0.00000000000000,
        5.23010455462158,
    ];

    let attyp = [6, 6, 6, 1, 1, 1, 1];
    let mut xtb = XtbModel::new(&attyp).with_charge(0.0).with_unpaired_electrons(0);
    let mut gradient = [0.0; 21];
    let energy = xtb.calculate_energy_and_gradient(&coord, &mut gradient)?;

    let dipole = xtb.get_dipole().unwrap();
    assert!((energy + 8.3824793849585).abs() < 1.0e-9);
    assert!((dipole[2] + 0.298279305689518).abs() < 1.0e-6);

    Ok(())
}
// 1e609bf6 ends here

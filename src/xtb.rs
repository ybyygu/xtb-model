// [[file:../xtb.note::a7b88800][a7b88800]]
use super::*;

use anyhow::*;
// a7b88800 ends here

// [[file:../xtb.note::1e609bf6][1e609bf6]]
#[test]
fn test_xtb_energy_and_gradient() {
    let natoms = 7;
    let attyp = [6, 6, 6, 1, 1, 1, 1];
    let charge = 0.0;
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
    let mut energy = 0.0;
    let mut dipole = [0.0; 3];
    let mut gradient = [0.0; 21];

    unsafe {
        let coord = coord.as_ptr();
        let attyp = attyp.as_ptr();
        let gradient = gradient.as_mut_ptr();
        let dipole = dipole.as_mut_ptr();
        let ret = xtb_calculate_energy_and_gradient(coord, attyp, natoms, charge, 0, &mut energy, gradient, dipole);
    }

    assert!((energy + 8.3824793849585).abs() < 1.0e-9);
    assert!((dipole[2] + 0.298279305689518).abs() < 1.0e-6);
}
// 1e609bf6 ends here

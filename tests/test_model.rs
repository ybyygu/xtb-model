// [[file:../xtb.note::6da62560][6da62560]]
use anyhow::*;
use xtb_model::test::ATOM_COORDS;
use xtb_model::{XtbModel, XtbParameters};

#[test]
fn test_xtb_raw_api() -> Result<()> {
    let coord = ATOM_COORDS;
    let attyp = [6, 6, 6, 1, 1, 1, 1];

    let mut xtb = XtbModel::create(&attyp, &coord, None)?;
    let mut gradient = coord.clone();
    let energy = xtb.calculate_energy_and_gradient(&mut gradient)?;
    let dipole = xtb.get_dipole().unwrap();

    assert!((energy + 8.3824793849585).abs() < 1.0e-9);
    assert!((dipole[2] + 0.298279305689518).abs() < 1.0e-6);
    dbg!(gradient);

    let mut params = XtbParameters::default();
    params.charge(0.0).unpaired_electrons(2).output_minimal();
    let xtb = XtbModel::create(&attyp, &coord, dbg!(params))?;

    Ok(())
}
// 6da62560 ends here

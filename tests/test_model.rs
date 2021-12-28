// [[file:../xtb.note::6da62560][6da62560]]
use anyhow::*;
use xtb_model::test::ATOM_COORDS;
use xtb_model::{XtbModel, XtbParameters};
use approx::assert_relative_eq;

#[test]
fn test_xtb_raw_api() -> Result<()> {
    let coord = ATOM_COORDS;
    let attyp = [6, 6, 6, 1, 1, 1, 1];

    let mut xtb = XtbModel::create(&attyp, &coord, None)?;
    let mut gradient = coord.clone();
    let energy = xtb.calculate_energy_and_gradient(&mut gradient)?;
    let dipole = xtb.get_dipole().unwrap();

    assert_relative_eq!(energy, -8.3824793849585, epsilon=1e-9);
    assert_relative_eq!(dipole[2], -0.298279305689518, epsilon=1e-6);

    let mut params = XtbParameters::default();
    params.charge(0.0).unpaired_electrons(0).output_muted().method("GFN1-xTB");
    let mut xtb = XtbModel::create(&attyp, &coord, dbg!(params))?;
    let energy = xtb.calculate_energy_and_gradient(&mut gradient)?;
    assert_relative_eq!(energy, -8.424757953815186, epsilon=1e-9);

    Ok(())
}
// 6da62560 ends here

// [[file:../xtb.note::1c066e9f][1c066e9f]]
use anyhow::*;
use approx::assert_relative_eq;
use xtb_model::libxtb::*;
use xtb_model::test::ATOM_COORDS;

#[test]
fn test_xtb_raw_api() -> Result<()> {
    let coord = ATOM_COORDS;
    let attyp = [6, 6, 6, 1, 1, 1, 1];

    let env = XtbEnvironment::new();
    let mol = XtbMolecule::create(&env, &attyp, &coord, 0.0, 0, None, None)?;
    let calc = XtbCalculator::new();
    calc.load_parametrization(&mol, &env, XtbMethod::GFN2xTB)?;
    let res = calc.single_point(&mol, &env)?;
    let energy = res.get_energy(&env)?;
    let dipole = res.get_dipole(&env)?;
    assert_relative_eq!(energy, -8.3824793849585, epsilon=1e-9);
    assert_relative_eq!(dipole[2], -0.298279305689518, epsilon=1e-6);

    calc.load_parametrization(&mol, &env, XtbMethod::GFN1xTB)?;
    let res = calc.single_point(&mol, &env)?;
    let energy = res.get_energy(&env)?;
    assert_relative_eq!(energy, -8.424757953815186, epsilon=1e-9);

    Ok(())
}
// 1c066e9f ends here

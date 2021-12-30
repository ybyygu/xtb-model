// [[file:../xtb.note::6da62560][6da62560]]
use anyhow::*;
use approx::assert_relative_eq;
use xtb_model::test::ATOM_COORDS;
use xtb_model::{XtbModel, XtbParameters};

#[test]
fn test_xtb_model() -> Result<()> {
    let coord = ATOM_COORDS;
    let attyp = [6, 6, 6, 1, 1, 1, 1];

    let mut xtb = XtbModel::create(&attyp, &coord, None)?;
    let mut gradient = coord.clone();
    let energy = xtb.calculate_energy_and_gradient(&mut gradient)?;
    let dipole = xtb.get_dipole().unwrap();

    assert_relative_eq!(energy, -8.3824793849585, epsilon = 1e-9);
    assert_relative_eq!(dipole[2], -0.298279305689518, epsilon = 1e-6);

    let mut params = XtbParameters::default();
    params
        .charge(0.0)
        .unpaired_electrons(0)
        .output_verbose()
        .method("GFN1-xTB");
    let mut xtb = XtbModel::create(&attyp, &coord, dbg!(params))?;
    let energy = xtb.calculate_energy_and_gradient(&mut gradient)?;
    assert_relative_eq!(energy, -8.424757953815186, epsilon = 1e-9);

    Ok(())
}

#[test]
#[ignore]
fn test_xtb_model_3d() -> Result<()> {
    let numbers = [6, 6, 6, 6, 6, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 7, 7, 7, 7];
    let coord = [
         9.77104501e-01,  1.24925555e-01,  8.22139769e+00,
         8.37995371e-01,  8.23489051e+00,  3.74893761e+00,
         4.62693404e+00, -2.45721089e+00,  8.22052352e+00,
         4.62532610e+00,  1.41051267e+00,  5.97940016e+00,
         9.71618351e-01,  1.17570237e-01,  3.75065164e+00,
        -2.80917006e+00,  6.94865315e+00,  5.99166085e+00,
         4.06610161e+00,  4.51252077e+00,  6.46827038e-01,
         2.76223056e-01, -8.50055887e-01,  2.06420987e+00,
         2.84806942e-01,  2.07039689e+00,  8.22836360e+00,
         2.90284064e+00,  8.22939158e+00,  3.73820878e+00,
         6.69188274e+00, -2.46191735e+00,  8.22593771e+00,
         6.69035555e+00,  1.41863696e+00,  5.97712614e+00,
         7.73011343e+00,  1.91963880e+00,  6.45533278e-01,
         3.94842571e+00,  3.36121142e+00,  5.97668593e+00,
        -3.49960564e+00,  5.97197638e+00,  7.67502785e+00,
         2.79250975e-01,  2.06298102e+00,  3.73907675e+00,
        -3.50586965e+00,  5.96534053e+00,  4.31491171e+00,
         1.56432603e-01,  7.25773353e+00,  2.06229892e+00,
        -4.98732693e-02,  6.88619344e+00,  5.98746725e+00,
        -4.50657119e-03, -1.16906911e+00,  5.98934273e+00,
         3.73678498e+00,  1.55157272e-01,  8.27155126e+00,
         3.73119434e+00,  1.47879860e-01,  3.69345547e+00,
    ];
    let lattice = [
         1.13437228e+01, -1.84405404e-03,  1.33836685e-05,
        -3.78300868e+00,  1.06992286e+01, -1.04202175e-03,
        -3.78025723e+00, -5.34955718e+00,  9.26593601e+00,
        ];

    let mut params = XtbParameters::default();
    params.output_muted().method("GFN-FF").lattice(lattice);
    let mut xtb = XtbModel::create(&numbers, &coord, params)?;
    let mut gradient = coord.clone();
    let energy = xtb.calculate_energy_and_gradient(&mut gradient)?;
    dbg!(energy);

    Ok(())
}
// 6da62560 ends here

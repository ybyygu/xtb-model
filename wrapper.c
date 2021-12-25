/* [[file:xtb.note::c29489e5][c29489e5]] */
/* adopted from test/api/c_api_example.c */
#include <assert.h>
#include <math.h>
#include <stdio.h>

#include "include/xtb.h"
/*
- coord: input cartesian coordinates in array with N atoms.
- attyp: input atom element type, in atomic number
- charge: input system charge
- uhf: the number of unpaired electrons
- natoms: total number of atoms
- gradient: a flatten array for output, in Nx3
 */
int xtb_calculate_energy_and_gradient(double const coord[], int const attyp[],
                                      int const natoms, double const charge,
                                      int const uhf, double *energy,
                                      double gradient[], double dipole[3]) {
  xtb_TEnvironment env;
  xtb_TMolecule mol;
  xtb_TCalculator calc;
  xtb_TResults res;
  /* int buffersize = 512; */
  /* char buffer[buffersize]; */

  assert(XTB_API_VERSION == xtb_getAPIVersion());

  env = xtb_newEnvironment();
  calc = xtb_newCalculator();
  res = xtb_newResults();
  mol = xtb_newMolecule(env,     /* env */
                        &natoms, /* natoms */
                        attyp,   /* numbers [natoms] */
                        coord,   /* positions [natoms][3] */
                        &charge, /* charge in e */
                        &uhf,    /* uhf, the number of unpaired electrons */
                        NULL,    /* lattice [3][3] */
                        NULL     /* periodic [3] */
  );
  if (xtb_checkEnvironment(env)) {
    xtb_showEnvironment(env, NULL);
    return 1;
  }

  xtb_setVerbosity(env, XTB_VERBOSITY_FULL);
  if (xtb_checkEnvironment(env)) {
    xtb_showEnvironment(env, NULL);
    return 2;
  }

  xtb_loadGFN2xTB(env, mol, calc, NULL);
  if (xtb_checkEnvironment(env)) {
    xtb_showEnvironment(env, NULL);
    return 3;
  }

  xtb_singlepoint(env, mol, calc, res);
  if (xtb_checkEnvironment(env)) {
    xtb_showEnvironment(env, NULL);
    return 4;
  }

  xtb_getEnergy(env, res, energy);
  xtb_getDipole(env, res, dipole);
  xtb_getGradient(env, res, gradient);
  if (xtb_checkEnvironment(env)) {
    xtb_showEnvironment(env, NULL);
    return 5;
  }

  xtb_delResults(&res);
  xtb_delCalculator(&calc);
  xtb_delMolecule(&mol);
  xtb_delEnvironment(&env);

  assert(!res);
  assert(!calc);
  assert(!mol);
  assert(!env);

  return 0;
}
/* c29489e5 ends here */

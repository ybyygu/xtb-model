/* [[file:xtb.note::736399d7][736399d7]] */
/*
- coord: input cartesian coordinates in array with N atoms.
- attyp: input atom element type, in atomic number
- charge: input system charge
- uhf: the number of unpaired electrons
- natoms: total number of atoms
- gradient: a flatten array for output, in Nx3
 */
extern int xtb_calculate_energy_and_gradient(double const coord[],
                                           int const attyp[], int const natoms,
                                           double const charge, int const uhf,
                                           double *energy, double gradient[],
                                           double dipole[3]);
/* 736399d7 ends here */

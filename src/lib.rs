// [[file:../xtb.note::3a96f242][3a96f242]]
#![allow(nonstandard_style)]

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub(crate) use bindings::*;
// 3a96f242 ends here

// [[file:../xtb.note::0a60241b][0a60241b]]
use anyhow::*;
// 0a60241b ends here

// [[file:../xtb.note::b6996cbf][b6996cbf]]
mod raw;
mod xtb;
// b6996cbf ends here

// [[file:../xtb.note::12b11409][12b11409]]
pub use crate::xtb::*;

/// Low level wrapper for xtb api
pub mod libxtb {
    pub use super::raw::*;
}

/// test data adopted from xtb-src/test/api/c_api_example.c
pub mod test {
    pub const ATOM_COORDS: [f64; 21] = [
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
    pub const ATOM_TYPES: [i32; 7] = [6, 6, 6, 1, 1, 1, 1];
}
// 12b11409 ends here

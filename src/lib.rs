// [[file:../xtb.note::3a96f242][3a96f242]]
#![allow(nonstandard_style)]

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub(crate) use bindings::*;
// 3a96f242 ends here

// [[file:../xtb.note::0a60241b][0a60241b]]

// 0a60241b ends here

// [[file:../xtb.note::*mods][mods:1]]
mod xtb;
// mods:1 ends here

// [[file:../xtb.note::*pub][pub:1]]
pub use crate::xtb::*;
// pub:1 ends here
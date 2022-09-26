#![doc = include_str!("../README.md")]

pub mod prelude {
    pub use crate::*;
}

mod root;
mod types;

pub use types::*;

pub use atoms::*;
pub use hooks::*;
pub use root::*;

mod atoms {
    mod atom;
    // mod atomfamily;
    // mod atomref;
    mod selector;
    // mod selectorfamily;

    pub use atom::*;
    // pub use atomfamily::*;
    // pub use atomref::*;
    pub use selector::*;
    // pub use selectorfamily::*;
}

pub mod hooks {
    // mod atom_ref;
    mod atom_root;
    mod init_atom_root;
    mod read;
    mod set;
    mod state;
    // pub use atom_ref::*;
    pub use atom_root::*;
    pub use init_atom_root::*;
    pub use read::*;
    pub use set::*;
    pub use state::*;
}

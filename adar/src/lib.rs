#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod state_machine;
pub mod tuples;
pub use adar_macros as macros;
pub mod enums;
mod utils;

pub mod prelude {
    pub use crate::enums::*;
    pub use crate::macros::*;
    pub use crate::state_machine::*;
    pub use crate::tuples::*;
}

#[cfg(feature = "alloc")]
pub use async_trait::async_trait;

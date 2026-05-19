#[cfg(feature = "split-facade")]
mod reta_split_abi;

#[cfg(feature = "split-facade")]
mod facade;

#[cfg(feature = "split-facade")]
pub use facade::*;

#[cfg(not(feature = "split-facade"))]
include!("lib_full.rs");

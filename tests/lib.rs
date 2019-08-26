#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(feature = "std")]
extern crate core;

#[macro_use]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg_attr(feature = "alloc", macro_use)]
extern crate safe_transmute;


mod error;
mod base;
mod bool;
mod full;
mod util;


include!("test_util/le_to_native.rs");
include!("test_util/aligned_vec.rs");

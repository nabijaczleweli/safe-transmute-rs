#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(feature = "std")]
extern crate core;

extern crate safe_transmute;


mod util;
mod from_bytes;
mod alignment_check;
mod from_bytes_pedantic;

mod base_transmute_vec;
mod base_transmute_many;
mod base_transmute_many_pedantic;

mod transmute_one;
mod transmute_one_pedantic;
mod transmute_many;
mod transmute_vec;
mod transmute_bool_pedantic;
mod transmute_bool_permissive;
mod transmute_bool_vec_pedantic;
mod transmute_bool_vec_permissive;
mod transmute_many_pedantic;
mod transmute_many_permissive;


include!("test_util/le_to_native.rs");
include!("test_util/aligned_vec.rs");

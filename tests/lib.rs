#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(feature = "std")]
extern crate core;

extern crate safe_transmute;


mod util;
mod from_bytes;
mod alignment_check;
mod from_bytes_pedantic;

mod guarded_transmute_vec;
mod guarded_transmute_many;
mod guarded_transmute_pod_vec;
mod guarded_transmute_vec_pedantic;
mod guarded_transmute_pod_pedantic;
mod guarded_transmute_many_pedantic;
mod guarded_transmute_vec_permissive;
mod guarded_transmute_many_permissive;
mod guarded_transmute_pod_vec_pedantic;
mod guarded_transmute_pod_vec_permissive;

mod safe_transmute_one;
mod safe_transmute_many;
mod safe_transmute_bool_pedantic;
mod safe_transmute_many_pedantic;
mod safe_transmute_bool_permissive;
mod safe_transmute_many_permissive;
mod safe_transmute_bool_vec_pedantic;
mod safe_transmute_bool_vec_permissive;


include!("test_util/le_to_native.rs");
include!("test_util/aligned_vec.rs");

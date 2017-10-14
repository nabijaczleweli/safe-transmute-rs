extern crate safe_transmute;


mod util;
mod guarded_transmute;
mod guarded_transmute_many;
mod guarded_transmute_pedantic;
mod guarded_transmute_many_pedantic;
mod guarded_transmute_many_permissive;
mod guarded_transmute_vec;
mod guarded_transmute_vec_pedantic;
mod guarded_transmute_vec_permissive;
mod guarded_transmute_pod;
mod guarded_transmute_pod_many;
mod guarded_transmute_pod_pedantic;
mod guarded_transmute_pod_many_pedantic;
mod guarded_transmute_pod_many_permissive;
mod guarded_transmute_pod_vec;
mod guarded_transmute_pod_vec_pedantic;
mod guarded_transmute_pod_vec_permissive;


include!("test_util/le_to_native.rs");

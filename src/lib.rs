//! This crate contains checked implementations of `transmute()`.
//!
//! The base functions in this crate are not inherently safe, but just guarded against common simple mistakes
//! (like trying to create an 8-byte type from 7 bytes).
//! These functions are exactly as safe as the data passed to them - creating a null pointer,
//! for example, is not unsafe in and of itself, but dereferencing it certainly *is*,
//! but they don't do that (see [here](https://github.com/nabijaczleweli/safe-transmute-rs/issues/1)
//! for extended discussion).
//!
//! Other functions in this crate, on the other hand, provide enough safety measures to ensure safety in
//! all circumstances. This is the case for those found in the `pod` and `bool` modules.
//!
//! Take note, however, that alignment is *unaccounted for*: you may, in the course of using this crate,
//! invoke unaligned access, which some CPUs *may* trap on; that did not, however, happen in any of our tests on
//! MIPS64 (BE), x86_64 (LE), nor armv6l (LE).
//!
//! # Examples
//!
//! View bytes as a series of `u16`s:
//!
//! ```
//! # use safe_transmute::guarded_transmute_many;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! # unsafe {
//! assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01,
//!                                            0x12, 0x34,
//!                                            // Spare byte, unused
//! # /*
//!                                            0x00])?,
//! # */
//! #                                          0x00].le_to_native::<u16>()).unwrap(),
//!            &[0x0100, 0x3412]);
//! # }
//! # }
//! ```
//!
//! View all bytes as a series of `u16`s:
//!
//! ```
//! # use safe_transmute::guarded_transmute_many_pedantic;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! # unsafe {
//! assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00, 0x01,
//! # /*
//!                                                     0x12, 0x34])?,
//! # */
//! #                                                   0x12, 0x34].le_to_native::<u16>()).unwrap(),
//!            &[0x0100, 0x3412]);
//! # }
//! # }
//! ```
//!
//! View bytes as an `f64`:
//!
//! ```
//! # use safe_transmute::guarded_transmute;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! # unsafe {
//! assert_eq!(guarded_transmute::<f64>(&[0x00, 0x00, 0x00, 0x00,
//! # /*
//!                                       0x00, 0x00, 0x00, 0x40])?,
//! # */
//! #                                     0x00, 0x00, 0x00, 0x40].le_to_native::<f64>()).unwrap(),
//!            2.0);
//! # }
//! # }
//! ```
//!
//! View a series of `u16`s as bytes:
//!
//! ```
//! # use safe_transmute::guarded_transmute_to_bytes_pod_many;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! # unsafe {
//! assert_eq!(guarded_transmute_to_bytes_pod_many(&[0x0001u16,
//!                                                  0x1234u16]),
//! # /*
//!            &[0x01, 0x00, 0x34, 0x12].le_to_native::<u16>());
//! # */
//! #          &[0x01, 0x00, 0x34, 0x12].le_to_native::<u16>());
//! # }
//! # }
//! ```


mod pod;
mod bool;
mod error;
mod to_bytes;

use std::slice;
use std::mem::{size_of, forget};
use guard::{SingleValueGuard, PermissiveGuard, SingleManyGuard, PedanticGuard, Guard};

pub mod util;
pub mod guard;

pub use self::error::{ErrorReason, GuardError, Error};
pub use self::to_bytes::{guarded_transmute_to_bytes_pod_many, guarded_transmute_to_bytes_many, guarded_transmute_to_bytes_pod, guarded_transmute_to_bytes};
pub use self::pod::{PodTransmutable, guarded_transmute_pod_many_permissive, guarded_transmute_pod_vec_permissive, guarded_transmute_pod_many_pedantic,
                    guarded_transmute_pod_vec_pedantic, guarded_transmute_pod_pedantic, guarded_transmute_pod_many, guarded_transmute_pod_vec,
                    guarded_transmute_pod};
pub use self::bool::{guarded_transmute_bool_vec_permissive, guarded_transmute_bool_vec_pedantic, guarded_transmute_bool_permissive,
                     guarded_transmute_bool_pedantic};


/// Transmute a byte slice into a single instance of a `Copy`able type.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01])?, 0x01000000);
/// # */
/// # assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x01000000);
/// # }
/// # }
/// ```
pub unsafe fn guarded_transmute<T: Copy>(bytes: &[u8]) -> Result<T, Error> {
    SingleManyGuard::check::<T>(bytes)?;
    Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, 1)[0])
}

/// Transmute a byte slice into a single instance of a `Copy`able type.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_pedantic::<u16>(&[0x0F, 0x0E])?, 0x0E0F);
/// # */
/// # assert_eq!(guarded_transmute_pedantic::<u16>(&[0x0F, 0x0E].le_to_native::<u16>()).unwrap(), 0x0E0F);
/// # }
/// # }
/// ```
pub unsafe fn guarded_transmute_pedantic<T: Copy>(bytes: &[u8]) -> Result<T, Error> {
    SingleValueGuard::check::<T>(bytes)?;
    Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, 1)[0])
}

/// View a byte slice as a slice of an arbitrary type.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_many;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            &[0x0100, 0x0200]);
/// # }
/// # }
/// ```
pub unsafe fn guarded_transmute_many<T>(bytes: &[u8]) -> Result<&[T], Error> {
    SingleManyGuard::check::<T>(bytes)?;
    Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / size_of::<T>()))
}

/// View a byte slice as a slice of an arbitrary type.
///
/// The resulting slice will have as many instances of a type as will fit, rounded down.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_many_permissive;
/// # unsafe {
/// assert_eq!(guarded_transmute_many_permissive::<u16>(&[0x00]), &[]);
/// # }
/// ```
pub unsafe fn guarded_transmute_many_permissive<T>(bytes: &[u8]) -> &[T] {
    PermissiveGuard::check::<T>(bytes).unwrap();
    slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / size_of::<T>())
}

/// View a byte slice as a slice of an arbitrary type.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// and should not have extraneous data.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_many_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B])?,
/// # */
/// # assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B].le_to_native::<u16>()).unwrap(),
///            &[0x0E0F, 0x0B0A]);
/// # }
/// # }
/// ```
pub unsafe fn guarded_transmute_many_pedantic<T>(bytes: &[u8]) -> Result<&[T], Error> {
    PedanticGuard::check::<T>(bytes)?;
    Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / size_of::<T>()))
}

/// Trasform a byte vector into a vector of an arbitrary type.
///
/// The resulting vec will reuse the allocated byte buffer when possible, and
/// should have at least enough bytes to fill a single instance of a type.
/// Extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_vec;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_vec::<u16>(vec![0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(guarded_transmute_vec::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
/// # /*
/// assert_eq!(guarded_transmute_vec::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED])?,
/// # */
/// # assert_eq!(guarded_transmute_vec::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()).unwrap(),
///            vec![0x00000004]);
///
/// assert!(guarded_transmute_vec::<i16>(vec![0xED]).is_err());
/// # }
/// # }
/// ```
pub unsafe fn guarded_transmute_vec<T>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    SingleManyGuard::check::<T>(&bytes)?;
    Ok(guarded_transmute_vec_permissive(bytes))
}

/// Trasform a byte vector into a vector of an arbitrary type.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// have as many instances of a type as will fit, rounded down.
/// Extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_vec_permissive;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02]),
/// # */
/// # assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
///            vec![0x0100, 0x0200]);
/// # /*
/// assert_eq!(guarded_transmute_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]),
/// # */
/// # assert_eq!(guarded_transmute_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()),
///            vec![0x00000004]);
/// assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0xED]), vec![]);
/// # }
/// # }
/// ```
pub unsafe fn guarded_transmute_vec_permissive<T>(mut bytes: Vec<u8>) -> Vec<T> {
    PermissiveGuard::check::<T>(&bytes).unwrap();
    let ptr = bytes.as_mut_ptr();
    let capacity = bytes.capacity() / size_of::<T>();
    let len = bytes.len() / size_of::<T>();
    forget(bytes);
    Vec::from_raw_parts(ptr as *mut T, capacity, len)
}

/// Trasform a byte vector into a vector of an arbitrary type.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not have extraneous data.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_vec_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(guarded_transmute_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
///
/// assert!(guarded_transmute_vec_pedantic::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]).is_err());
/// # }
/// # }
/// ```
pub unsafe fn guarded_transmute_vec_pedantic<T>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    PedanticGuard::check::<T>(&bytes)?;
    Ok(guarded_transmute_vec_permissive(bytes))
}

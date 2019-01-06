//! Module for functions which ensure full memory safety.
//!
//! Functions in this module are guarded from out-of-bounds memory access as
//! well as from unaligned access, returning errors on both cases. Moreover,
//! only a [`PodTransmutable`](trait.PodTransmutable.html)) can be used as the
//! transmute target, thus ensuring full safety.
//! 
//! Unless this was previously imposed by certain means, the functions in this
//! module may arbitrarily fail due to unaligned memory access. It is up to the
//! user of this crate to make the receiving data well aligned for the intended
//! target type.

use crate::Error;
use crate::guard::{Guard, PedanticGuard, PermissiveGuard, SingleValueGuard};
#[cfg(feature = "std")]
use crate::pod::{guarded_transmute_pod_vec};
use crate::pod::{PodTransmutable, guarded_transmute_pod, guarded_transmute_pod_many};
use crate::align::check_alignment;


/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not have enough bytes for a single value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::safe_transmute_one;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(safe_transmute_one::<u32>(&[0x00, 0x00, 0x00, 0x01])?, 0x0100_0000);
/// # */
/// # assert_eq!(safe_transmute_one::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x0100_0000);
/// # }
/// ```
pub fn safe_transmute_one<T: PodTransmutable>(bytes: &[u8]) -> Result<T, Error> {
    check_alignment::<_, T>(bytes)?;
    unsafe { guarded_transmute_pod(bytes) }
}

/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not have enough bytes for a single value `T`.
/// - The data has more bytes than those required to produce a single value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::safe_transmute_one_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(safe_transmute_one_pedantic::<u16>(&[0x0F, 0x0E])?, 0x0E0F);
/// # */
/// # assert_eq!(safe_transmute_one_pedantic::<u16>(&[0x0F, 0x0E].le_to_native::<u16>()).unwrap(), 0x0E0F);
/// # }
/// ```
pub fn safe_transmute_one_pedantic<T: PodTransmutable>(bytes: &[u8]) -> Result<T, Error> {
    SingleValueGuard::check::<T>(bytes)?;
    check_alignment::<_, T>(bytes)?;
    unsafe { guarded_transmute_pod(bytes) }
}

/// Transmute a byte slice into a sequence of values of the given type.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not have enough bytes for a single value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::{SingleManyGuard, safe_transmute_many};
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(safe_transmute_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(safe_transmute_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            &[0x0100, 0x0200]);
/// # }
/// ```
pub fn safe_transmute_many<T: PodTransmutable, G: Guard>(bytes: &[u8]) -> Result<&[T], Error> {
    check_alignment::<_, T>(bytes)?;
    unsafe { guarded_transmute_pod_many::<_, G>(bytes) }
}

/// Transmute a byte slice into a sequence of values of the given type.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// 
/// # Examples
///
/// ```no_run
/// # use safe_transmute::safe_transmute_many_permissive;
/// assert_eq!(safe_transmute_many_permissive::<u16>(&[0x00]), Ok([].as_ref()));
/// ```
pub fn safe_transmute_many_permissive<T: PodTransmutable>(bytes: &[u8]) -> Result<&[T], Error> {
    safe_transmute_many::<T, PermissiveGuard>(bytes)
}

/// Transmute a byte slice into a sequence of values of the given type.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not have enough bytes for a single value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::safe_transmute_many_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(safe_transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B])?,
/// # */
/// # assert_eq!(safe_transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B].le_to_native::<u16>()).unwrap(),
///            &[0x0E0F, 0x0B0A]);
/// # }
/// ```
pub fn safe_transmute_many_pedantic<T: PodTransmutable>(bytes: &[u8]) -> Result<&[T], Error> {
    safe_transmute_many::<T, PedanticGuard>(bytes)
}

/// Transform a byte vector into a vector of values.
///
/// The resulting vec will reuse the allocated byte buffer when successful.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not comply with the given memory guard strategy.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::{safe_transmute_vec, SingleManyGuard};
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(safe_transmute_vec::<u16, SingleManyGuard>(vec![0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(safe_transmute_vec::<u16, SingleManyGuard>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
/// # /*
/// assert_eq!(safe_transmute_vec::<u32, SingleManyGuard>(vec![0x04, 0x00, 0x00, 0x00, 0xED])?,
/// # */
/// # assert_eq!(safe_transmute_vec::<u32, SingleManyGuard>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()).unwrap(),
///            vec![0x0000_0004]);
///
/// assert!(safe_transmute_vec::<i16, SingleManyGuard>(vec![0xED]).is_err());
/// # }
/// ```
#[cfg(feature = "std")]
pub fn safe_transmute_vec<T: PodTransmutable, G: Guard>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    check_alignment::<_, T>(&bytes)?;
    unsafe { guarded_transmute_pod_vec::<T, G>(bytes) }
}

/// Transform a byte vector into a vector of values.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// have as many instances of a type as will fit, rounded down.
/// Extraneous data is ignored.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::safe_transmute_vec_permissive;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(safe_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02]),
/// # */
/// # assert_eq!(safe_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
///            Ok(vec![0x0100, 0x0200]));
/// # /*
/// assert_eq!(safe_transmute_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]),
/// # */
/// # assert_eq!(safe_transmute_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()),
///            Ok(vec![0x0000_0004]));
/// assert_eq!(safe_transmute_vec_permissive::<u16>(vec![0xED]), Ok(vec![]));
/// # }
/// ```
#[cfg(feature = "std")]
pub fn safe_transmute_vec_permissive<T: PodTransmutable>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    safe_transmute_vec::<T, PermissiveGuard>(bytes)
}

/// Transform a byte vector into a vector of values.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not have extraneous data.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not have enough bytes for a single value `T`.
/// - The last chunk of the size of `T` is not large enough for a value, leaving extraneous bytes.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::safe_transmute_vec_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(safe_transmute_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(safe_transmute_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
///
/// assert!(safe_transmute_vec_pedantic::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]).is_err());
/// # }
/// ```
#[cfg(feature = "std")]
pub fn safe_transmute_vec_pedantic<T: PodTransmutable>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    safe_transmute_vec::<T, PedanticGuard>(bytes)
}

//! Module for functions which ensure full memory safety.
//!
//! Functions in this module are guarded from out-of-bounds memory access as
//! well as from unaligned access, returning errors on both cases. Moreover,
//! only a [`TriviallyTransmutable`](trait.TriviallyTransmutable.html)) can be
//! used as the transmute target, thus ensuring full safety.
//!
//! Unless this was previously imposed by certain means, the functions in this
//! module may arbitrarily fail due to unaligned memory access. It is up to the
//! user of this crate to make the receiving data well aligned for the intended
//! target type.


use self::super::trivial::{TriviallyTransmutable, transmute_trivial_many, transmute_trivial};
use self::super::guard::{SingleValueGuard, PermissiveGuard, PedanticGuard, Guard};
use self::super::Error;
#[cfg(feature = "std")]
use self::super::error::IncompatibleVecTargetError;
use self::super::align::check_alignment;


/// Transmute a byte slice into a single instance of a trivially transmutable type.
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
/// # use safe_transmute::transmute_one;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(transmute_one::<u32>(&[0x00, 0x00, 0x00, 0x01])?, 0x0100_0000);
/// # */
/// # assert_eq!(transmute_one::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x0100_0000);
/// # }
/// ```
pub fn transmute_one<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<T, Error<u8, T>> {
    check_alignment::<_, T>(bytes)?;
    unsafe { transmute_trivial(bytes) }
}

/// Transmute a byte slice into a single instance of a trivially transmutable type.
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
/// # use safe_transmute::transmute_one_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(transmute_one_pedantic::<u16>(&[0x0F, 0x0E])?, 0x0E0F);
/// # */
/// # assert_eq!(transmute_one_pedantic::<u16>(&[0x0F, 0x0E].le_to_native::<u16>()).unwrap(), 0x0E0F);
/// # }
/// ```
pub fn transmute_one_pedantic<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<T, Error<u8, T>> {
    SingleValueGuard::check::<T>(bytes)?;
    check_alignment::<_, T>(bytes)?;
    unsafe { transmute_trivial(bytes) }
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
/// # use safe_transmute::{SingleManyGuard, transmute_many};
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(transmute_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(transmute_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            &[0x0100, 0x0200]);
/// # }
/// ```
pub fn transmute_many<T: TriviallyTransmutable, G: Guard>(bytes: &[u8]) -> Result<&[T], Error<u8, T>> {
    check_alignment::<_, T>(bytes)?;
    unsafe { transmute_trivial_many::<_, G>(bytes) }
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
/// # use safe_transmute::transmute_many_permissive;
/// assert_eq!(transmute_many_permissive::<u16>(&[0x00]), Ok([].as_ref()));
/// ```
pub fn transmute_many_permissive<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<&[T], Error<u8, T>> {
    transmute_many::<T, PermissiveGuard>(bytes)
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
/// # use safe_transmute::transmute_many_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B])?,
/// # */
/// # assert_eq!(transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B].le_to_native::<u16>()).unwrap(),
///            &[0x0E0F, 0x0B0A]);
/// # }
/// ```
pub fn transmute_many_pedantic<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<&[T], Error<u8, T>> {
    transmute_many::<T, PedanticGuard>(bytes)
}

/// Transform a byte vector into a vector of values.
///
/// The resulting vector will reuse the allocated byte buffer when successful.
/// 
/// # Errors
///
/// An error is returned if _either_ the size or the minimum memory 
/// requirements are not the same between `S` and `U`:
/// 
/// - `std::mem::size_of::<S>() != std::mem::size_of::<T>()`
/// - `std::mem::align_of::<S>() != std::mem::align_of::<T>()`
/// 
/// Otherwise, the only truly safe way of doing this is to create a transmuted
/// slice view of the vector, or make a copy anyway. The
/// [`IncompatibleVecTargetError`](../error/struct.IncompatibleVecTargetError.html) error
/// type provides a means of making this copy to the intended target type.
///
/// # Examples
///
/// ```
/// # use safe_transmute::transmute_vec;
/// # fn run() -> Result<(), Box<::std::error::Error>> {
/// assert_eq!(transmute_vec::<u8, i8>(vec![0x00, 0x01, 0x00, 0x02])?,
///            vec![0x00i8, 0x01i8, 0x00i8, 0x02i8]);
/// assert_eq!(transmute_vec::<u8, i8>(vec![0x04, 0x00, 0x00, 0x00, 0xED])?,
///            vec![0x04, 0x00, 0x00, 0x00, 0xEDi8]);
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
#[cfg(feature = "std")]
pub fn transmute_vec<S: TriviallyTransmutable, T: TriviallyTransmutable>(mut vec: Vec<S>) -> Result<Vec<T>, Error<S, T>> {
    if ::std::mem::align_of::<S>() != ::std::mem::align_of::<T>() || ::std::mem::size_of::<S>() != ::std::mem::size_of::<T>() {
        return Err(IncompatibleVecTargetError::new(vec).into());
    }

    unsafe {
        let capacity = vec.capacity();
        let len = vec.len();
        let ptr = vec.as_mut_ptr();
        ::std::mem::forget(vec);
        Ok(Vec::from_raw_parts(ptr as *mut T, len, capacity))
    }
}

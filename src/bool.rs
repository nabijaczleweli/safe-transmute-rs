//! Functions for safe transmutation to `bool`.
//! 
//! Transmuting to `bool` is not undefined behavior if the transmuted value is
//! either 0 or 1. These functions will return an error if the integer value
//! behind the `bool` value is neither one.


use crate::base::guarded_transmute_many;
use crate::guard::{Guard, PedanticGuard, PermissiveGuard};
use crate::Error;
#[cfg(feature = "std")]
use crate::base::guarded_transmute_vec;
use core::mem::{size_of, transmute};


/// Makes sure that the bytes represent a sequence of valid boolean values.
///
/// # Panics
///
/// This shouldn't happen on all currently supported platforms, but this
/// function panics if the size of `bool` isn't 1.
#[inline]
pub fn bytes_are_bool(v: &[u8]) -> bool {
    // TODO make this a static assert once available
    assert_eq!(size_of::<bool>(),
               1,
               "unsupported platform due to invalid bool size {}, please report over at https://github.com/nabijaczleweli/safe-transmute-rs/issues/new",
               size_of::<bool>());

    v.iter().cloned().all(byte_is_bool)
}

#[inline]
fn byte_is_bool(b: u8) -> bool {
    unsafe { b == transmute::<_, u8>(false) || b == transmute::<_, u8>(true) }
}

fn guarded_transmute_bool<G: Guard>(bytes: &[u8]) -> Result<&[bool], Error>
{
    check_bool(bytes)?;
    unsafe { guarded_transmute_many::<_, G>(bytes) }
}

/// View a byte slice as a slice of boolean values.
///
/// The resulting slice will have as many instances of `bool` as will fit, can be empty.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, safe_transmute_bool_permissive};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(safe_transmute_bool_permissive(&[0x00, 0x01, 0x00, 0x01])?,
///            &[false, true, false, true]);
/// assert_eq!(safe_transmute_bool_permissive(&[])?, &[]);
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
pub fn safe_transmute_bool_permissive(bytes: &[u8]) -> Result<&[bool], Error> {
    guarded_transmute_bool::<PermissiveGuard>(bytes)
}

/// View a byte slice as a slice of boolean values.
///
/// The resulting slice will have as many instances of `bool` as will fit, can be empty.
#[deprecated(since = "0.11.0", note = "use `safe_transmute_bool_permissive()` instead")]
pub fn guarded_transmute_bool_permissive(bytes: &[u8]) -> Result<&[bool], Error> {
    guarded_transmute_bool::<PermissiveGuard>(bytes)
}

/// View a byte slice as a slice of boolean values.
///
/// The byte slice must have at least enough bytes to fill a single `bool`.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, safe_transmute_bool_pedantic};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(safe_transmute_bool_pedantic(&[0x01, 0x01, 0x01, 0x01])?,
///            &[true, true, true, true]);
/// assert!(safe_transmute_bool_pedantic(&[]).is_err());
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
pub fn safe_transmute_bool_pedantic(bytes: &[u8]) -> Result<&[bool], Error> {
    guarded_transmute_bool::<PedanticGuard>(bytes)
}

/// View a byte slice as a slice of boolean values.
///
/// The byte slice must have at least enough bytes to fill a single `bool`.
#[deprecated(since = "0.11.0", note = "use `safe_transmute_bool_pedantic()` instead")]
pub fn guarded_transmute_bool_pedantic(bytes: &[u8]) -> Result<&[bool], Error> {
    safe_transmute_bool_pedantic(bytes)
}

/// Trasform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// have as many instances of a type as will fit, rounded down.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, safe_transmute_bool_vec_permissive};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(safe_transmute_bool_vec_permissive(vec![0x00, 0x01, 0x00, 0x01])?,
///            vec![false, true, false, true]);
/// assert_eq!(safe_transmute_bool_vec_permissive(vec![0x01, 0x00, 0x00, 0x00, 0x01])?,
///            vec![true, false, false, false, true]);
/// assert_eq!(safe_transmute_bool_vec_permissive(vec![]), Ok(vec![]));
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
#[cfg(feature = "std")]
pub fn safe_transmute_bool_vec_permissive(bytes: Vec<u8>) -> Result<Vec<bool>, Error> {
    check_bool(&bytes)?;
    // Alignment guarantees are ensured, and all values have been checked,
    // so the conversion is safe.
    unsafe { guarded_transmute_vec::<_, PermissiveGuard>(bytes) }
}

/// Trasform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// have as many instances of a type as will fit, rounded down.
#[cfg(feature = "std")]
#[deprecated(since = "0.11.0", note = "use `safe_transmute_bool_vec_permissive()` instead")]
pub fn guarded_transmute_bool_vec_permissive(bytes: Vec<u8>) -> Result<Vec<bool>, Error> {
    safe_transmute_bool_vec_permissive(bytes)
}

/// Transform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not have extraneous data.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, safe_transmute_bool_vec_pedantic};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(safe_transmute_bool_vec_pedantic(vec![0x00, 0x01, 0x00, 0x01])?,
///            vec![false, true, false, true]);
///
/// assert!(safe_transmute_bool_vec_pedantic(vec![]).is_err());
///
/// assert!(safe_transmute_bool_vec_pedantic(vec![0x04, 0x00, 0xED]).is_err());
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
#[cfg(feature = "std")]
pub fn safe_transmute_bool_vec_pedantic(bytes: Vec<u8>) -> Result<Vec<bool>, Error> {
    check_bool(&bytes)?;
    // alignment guarantees are ensured, and all values have been checked,
    // so the conversion is safe.
    unsafe { guarded_transmute_vec::<_, PedanticGuard>(bytes) }
}

/// Transform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not have extraneous data.
#[cfg(feature = "std")]
#[deprecated(since = "0.11.0", note = "use `safe_transmute_bool_vec_pedantic()` instead")]
pub fn guarded_transmute_bool_vec_pedantic(bytes: Vec<u8>) -> Result<Vec<bool>, Error> {
    safe_transmute_bool_vec_pedantic(bytes)
}

/// Helper function for returning an error if any of the bytes does not make a
/// valid `bool`.
fn check_bool(bytes: &[u8]) -> Result<(), Error> {
    if bytes_are_bool(bytes) {
        Ok(())
    } else {
        Err(Error::InvalidValue)
    }
}

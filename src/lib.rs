//! This crate contains checked implementations of `transmute()`.
//!
//! The functions in this crate are not inherently safe, but just guarded against common simple mistakes
//! (like trying to create an 8-byte type from 7 bytes).
//!
//! Those functions are exactly as safe as the data passed to them - creating a null pointer,
//! for example, is not unsafe in and of itself, but dereferencing it certainly *is*,
//! but they don't do that (see [here](https://github.com/nabijaczleweli/safe-transmute-rs/issues/1)
//! for extended discussion).
//!
//! # Examples
//!
//! View bytes as a series of `u16`s:
//!
//! ```
//! # use safe_transmute::guarded_transmute_many;
//! # unsafe {
//! assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01,
//!                                            0x12, 0x34,
//!                                            // Spare byte, unused
//!                                            0x00]).unwrap(),
//!            &[0x0100, 0x3412]);
//! # }
//! ```
//!
//! View all bytes as a series of `u16`s:
//!
//! ```
//! # use safe_transmute::guarded_transmute_many_pedantic;
//! # unsafe {
//! assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00, 0x01,
//!                                                     0x12, 0x34]).unwrap(),
//!            &[0x0100, 0x3412]);
//! # }
//! ```
//!
//! View bytes as an `f64`:
//!
//! ```
//! # use safe_transmute::guarded_transmute;
//! # unsafe {
//! assert_eq!(guarded_transmute::<f64>(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap(),
//!            0.0);
//! # }
//! ```


mod error;

use std::slice;
use std::mem::align_of;

pub use self::error::{ErrorReason, Error};


/// Transmute a byte slice into a single instance of a `Copy`able type
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute;
/// // Little-endian
/// # unsafe {
/// assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01]).unwrap(), 0x01000000);
/// # }
/// ```
pub unsafe fn guarded_transmute<T: Copy>(bytes: &[u8]) -> Result<T, Error> {
    if bytes.len() < align_of::<T>() {
        Err(Error {
            required: align_of::<T>(),
            actual: bytes.len(),
            reason: ErrorReason::NotEnoughBytes,
        })
    } else {
        Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, 1)[0])
    }
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
/// // Little-endian
/// # unsafe {
/// assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02]).unwrap(), &[0x0100, 0x0200]);
/// # }
/// ```
pub unsafe fn guarded_transmute_many<T>(bytes: &[u8]) -> Result<&[T], Error> {
    if bytes.len() < align_of::<T>() {
        Err(Error {
            required: align_of::<T>(),
            actual: bytes.len(),
            reason: ErrorReason::NotEnoughBytes,
        })
    } else {
        Ok(guarded_transmute_many_permissive(bytes))
    }
}

/// View a byte slice as a slice of an arbitrary type.
///
/// The resulting slice will have as many instances of a type as will fit, rounded down.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_many_permissive;
/// // Little-endian
/// # unsafe {
/// assert_eq!(guarded_transmute_many_permissive::<u16>(&[0x00]), &[]);
/// # }
/// ```
pub unsafe fn guarded_transmute_many_permissive<T>(bytes: &[u8]) -> &[T] {
    slice::from_raw_parts(bytes.as_ptr() as *const T, (bytes.len() - (bytes.len() % align_of::<T>())) / align_of::<T>())
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
/// // Little-endian
/// # unsafe {
/// assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B]).unwrap(), &[0x0E0F, 0x0B0A]);
/// # }
/// ```
pub unsafe fn guarded_transmute_many_pedantic<T>(bytes: &[u8]) -> Result<&[T], Error> {
    if bytes.len() % align_of::<T>() != 0 {
        Err(Error {
            required: align_of::<T>(),
            actual: bytes.len(),
            reason: ErrorReason::InexactByteCount,
        })
    } else {
        Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / align_of::<T>()))
    }
}

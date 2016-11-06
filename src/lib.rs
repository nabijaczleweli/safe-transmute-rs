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
//! assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01,
//!                                            0x12, 0x34,
//!                                            // Spare byte, unused
//!                                            0x00]),
//!            &[0x0100, 0x3412]);
//! ```
//!
//! View bytes as an `f64`:
//!
//! ```
//! # use safe_transmute::guarded_transmute;
//! assert_eq!(guarded_transmute::<f64>(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
//!            0.0);
//! ```


use std::mem::align_of;
use std::slice;


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
/// assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01]), 0x01000000);
/// ```
pub unsafe fn guarded_transmute<T: Copy>(bytes: &[u8]) -> T {
    assert!(bytes.len() >= align_of::<T>(), "Not enough bytes to fill type");
    unsafe { slice::from_raw_parts(bytes.as_ptr() as *const T, 1)[0] }
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
/// assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02]), &[0x0100, 0x0200]);
/// ```
pub unsafe fn guarded_transmute_many<T>(bytes: &[u8]) -> &[T] {
    assert!(bytes.len() >= align_of::<T>(), "Not enough bytes to fill type");
    unsafe { slice::from_raw_parts(bytes.as_ptr() as *const T, (bytes.len() - (bytes.len() % align_of::<T>())) / align_of::<T>()) }
}

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
//! # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
//! # impl<'a> LeToNative for &'a mut [u8] {
//! #     #[cfg(target_endian = "little")]
//! #     fn le_to_native<T: Sized>(self) -> Self { self }
//! #     #[cfg(target_endian = "big")]
//! #     fn le_to_native<T: Sized>(mut self) -> Self {
//! #         use std::mem::size_of;
//! #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
//! #         self
//! #     }
//! # }
//! # impl LeToNative for [u8; 5] {
//! #     #[cfg(target_endian = "little")]
//! #     fn le_to_native<T: Sized>(self) -> Self { self }
//! #     #[cfg(target_endian = "big")]
//! #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
//! # }
//! # unsafe {
//! assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01,
//!                                            0x12, 0x34,
//!                                            // Spare byte, unused
//! # /*
//!                                            0x00]).unwrap(),
//! # */
//! #                                          0x00].le_to_native::<u16>()).unwrap(),
//!            &[0x0100, 0x3412]);
//! # }
//! ```
//!
//! View all bytes as a series of `u16`s:
//!
//! ```
//! # use safe_transmute::guarded_transmute_many_pedantic;
//! # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
//! # impl<'a> LeToNative for &'a mut [u8] {
//! #     #[cfg(target_endian = "little")]
//! #     fn le_to_native<T: Sized>(self) -> Self { self }
//! #     #[cfg(target_endian = "big")]
//! #     fn le_to_native<T: Sized>(mut self) -> Self {
//! #         use std::mem::size_of;
//! #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
//! #         self
//! #     }
//! # }
//! # impl LeToNative for [u8; 4] {
//! #     #[cfg(target_endian = "little")]
//! #     fn le_to_native<T: Sized>(self) -> Self { self }
//! #     #[cfg(target_endian = "big")]
//! #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
//! # }
//! # unsafe {
//! assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00, 0x01,
//! # /*
//!                                                     0x12, 0x34]).unwrap(),
//! # */
//! #                                                   0x12, 0x34].le_to_native::<u16>()).unwrap(),
//!            &[0x0100, 0x3412]);
//! # }
//! ```
//!
//! View bytes as an `f64`:
//!
//! ```
//! # use safe_transmute::guarded_transmute;
//! # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
//! # impl<'a> LeToNative for &'a mut [u8] {
//! #     #[cfg(target_endian = "little")]
//! #     fn le_to_native<T: Sized>(self) -> Self { self }
//! #     #[cfg(target_endian = "big")]
//! #     fn le_to_native<T: Sized>(mut self) -> Self {
//! #         use std::mem::size_of;
//! #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
//! #         self
//! #     }
//! # }
//! # impl LeToNative for [u8; 8] {
//! #     #[cfg(target_endian = "little")]
//! #     fn le_to_native<T: Sized>(self) -> Self { self }
//! #     #[cfg(target_endian = "big")]
//! #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
//! # }
//! # unsafe {
//! assert_eq!(guarded_transmute::<f64>(&[0x00, 0x00, 0x00, 0x00,
//! # /*
//!                                       0x00, 0x00, 0x00, 0x40]).unwrap(),
//! # */
//! #                                     0x00, 0x00, 0x00, 0x40].le_to_native::<f64>()).unwrap(),
//!            2.0);
//! # }
//! ```


mod pod;
mod error;

use std::slice;
use std::mem::{align_of, forget};

pub mod util;

pub use self::error::{ErrorReason, Error};
pub use self::pod::{PodTransmutable, guarded_transmute_pod_many_permissive, guarded_transmute_pod_vec_permissive, guarded_transmute_pod_many_pedantic,
                    guarded_transmute_pod_pedantic, guarded_transmute_pod_vec_pedantic, guarded_transmute_pod_many, guarded_transmute_pod,
                    guarded_transmute_pod_vec};


/// Transmute a byte slice into a single instance of a `Copy`able type.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute;
/// # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
/// # impl<'a> LeToNative for &'a mut [u8] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self {
/// #         use std::mem::size_of;
/// #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
/// #         self
/// #     }
/// # }
/// # impl LeToNative for [u8; 4] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
/// # }
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01]).unwrap(), 0x01000000);
/// # */
/// # assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x01000000);
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

/// Transmute a byte slice into a single instance of a `Copy`able type.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pedantic;
/// # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
/// # impl<'a> LeToNative for &'a mut [u8] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self {
/// #         use std::mem::size_of;
/// #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
/// #         self
/// #     }
/// # }
/// # impl LeToNative for [u8; 2] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
/// # }
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_pedantic::<u16>(&[0x0F, 0x0E]).unwrap(), 0x0E0F);
/// # */
/// # assert_eq!(guarded_transmute_pedantic::<u16>(&[0x0F, 0x0E].le_to_native::<u16>()).unwrap(), 0x0E0F);
/// # }
/// ```
pub unsafe fn guarded_transmute_pedantic<T: Copy>(bytes: &[u8]) -> Result<T, Error> {
    if bytes.len() != align_of::<T>() {
        Err(Error {
            required: align_of::<T>(),
            actual: bytes.len(),
            reason: ErrorReason::InexactByteCount,
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
/// # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
/// # impl<'a> LeToNative for &'a mut [u8] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self {
/// #         use std::mem::size_of;
/// #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
/// #         self
/// #     }
/// # }
/// # impl LeToNative for [u8; 4] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
/// # }
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            &[0x0100, 0x0200]);
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
/// # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
/// # impl<'a> LeToNative for &'a mut [u8] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self {
/// #         use std::mem::size_of;
/// #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
/// #         self
/// #     }
/// # }
/// # impl LeToNative for [u8; 4] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
/// # }
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B].le_to_native::<u16>()).unwrap(),
///            &[0x0E0F, 0x0B0A]);
/// # }
/// ```
pub unsafe fn guarded_transmute_many_pedantic<T>(bytes: &[u8]) -> Result<&[T], Error> {
    if bytes.len() < align_of::<T>() {
        Err(Error {
            required: align_of::<T>(),
            actual: bytes.len(),
            reason: ErrorReason::NotEnoughBytes,
        })
    } else if bytes.len() % align_of::<T>() != 0 {
        Err(Error {
            required: align_of::<T>(),
            actual: bytes.len(),
            reason: ErrorReason::InexactByteCount,
        })
    } else {
        Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / align_of::<T>()))
    }
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
/// # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
/// # impl<'a> LeToNative for &'a mut [u8] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self {
/// #         use std::mem::size_of;
/// #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
/// #         self
/// #     }
/// # }
/// # impl LeToNative for Vec<u8> {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
/// # }
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_vec::<u16>(vec![0x00, 0x01, 0x00, 0x02]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_vec::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
/// # /*
/// assert_eq!(guarded_transmute_vec::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_vec::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()).unwrap(),
///            vec![0x00000004]);
///
/// assert!(guarded_transmute_vec::<i16>(vec![0xED]).is_err());
/// # }
/// ```
pub unsafe fn guarded_transmute_vec<T>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    if bytes.len() < align_of::<T>() {
        Err(Error {
            required: align_of::<T>(),
            actual: bytes.len(),
            reason: ErrorReason::NotEnoughBytes,
        })
    } else {
        Ok(guarded_transmute_vec_permissive(bytes))
    }
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
/// # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
/// # impl<'a> LeToNative for &'a mut [u8] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self {
/// #         use std::mem::size_of;
/// #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
/// #         self
/// #     }
/// # }
/// # impl LeToNative for Vec<u8> {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
/// # }
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
/// ```
pub unsafe fn guarded_transmute_vec_permissive<T>(mut bytes: Vec<u8>) -> Vec<T> {
    let ptr = bytes.as_mut_ptr();
    let capacity = bytes.capacity() / align_of::<T>();
    let len = bytes.len() / align_of::<T>();
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
/// # trait LeToNative { fn le_to_native<T: Sized>(self) -> Self; }
/// # impl<'a> LeToNative for &'a mut [u8] {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self {
/// #         use std::mem::size_of;
/// #         for elem in self.chunks_mut(size_of::<T>()) { elem.reverse(); }
/// #         self
/// #     }
/// # }
/// # impl LeToNative for Vec<u8> {
/// #     #[cfg(target_endian = "little")]
/// #     fn le_to_native<T: Sized>(self) -> Self { self }
/// #     #[cfg(target_endian = "big")]
/// #     fn le_to_native<T: Sized>(mut self) -> Self { (&mut self[..]).le_to_native::<T>(); self }
/// # }
/// // Little-endian
/// # unsafe {
/// # /*
/// assert_eq!(guarded_transmute_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
///
/// assert!(guarded_transmute_vec_pedantic::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]).is_err());
/// # }
/// ```
pub unsafe fn guarded_transmute_vec_pedantic<T>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    let a = align_of::<T>();
    let len = bytes.len();
    if len < a {
        Err(Error {
            required: a,
            actual: len,
            reason: ErrorReason::NotEnoughBytes,
        })
    } else if len % a != 0 {
        Err(Error {
            required: a,
            actual: len,
            reason: ErrorReason::InexactByteCount,
        })
    } else {
        Ok(guarded_transmute_vec_permissive(bytes))
    }
}

//! The `guard` module exposes an API for memory boundary checking.
//!
//! # Examples:
//!
//! In order to check whether a value would fit in the given
//! slice without extraneous space:
//!
//! ```
//! # use safe_transmute::Error;
//! # use safe_transmute::guard::{Guard, SingleValueGuard};
//! # fn run() -> Result<(), Error> {
//! SingleValueGuard::check::<u32>(&[0x00, 0x01, 0x00, 0x02])?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! Different guard types implement different checking strategies.
//! For example, the pedantic guard type [`PedanticGuard`] requires
//! the slice to have space for at least one value, and not have
//! extraneous bytes at the end.
//!
//!
//! ```
//! # use safe_transmute::Error;
//! # use safe_transmute::guard::{Guard, PedanticGuard};
//! # fn run() -> Result<(), Error> {
//! PedanticGuard::check::<u16>(&[0xAA, 0xAA, 0xBB, 0xBB, 0xCC, 0xCC])?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! [`PermissiveGuard`], on the other hand, will accept any memory slice.
//!
//! ```
//! # use safe_transmute::Error;
//! # use safe_transmute::guard::{Guard, PermissiveGuard};
//! # fn run() -> Result<(), Error> {
//! PermissiveGuard::check::<i16>(b"covfefe")?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! If the check fails, the resulting [`GuardError`] value describes why.
//!
//! ```
//! # use safe_transmute::{GuardError, ErrorReason};
//! # use safe_transmute::guard::{Guard, PedanticGuard};
//! assert_eq!(
//!     PedanticGuard::check::<i16>(b"covfefe"),
//!     Err(GuardError {
//!         required: 2,
//!         actual: 7,
//!         reason: ErrorReason::InexactByteCount,
//!     }));
//! ```
//!
//! [`GuardError`]: ../type.GuardError.html
//! [`PedanticGuard`]: struct.PedanticGuard.html
//! [`PermissiveGuard`]: struct.PermissiveGuard.html

use error::{ErrorReason, GuardError};
use std::mem::align_of;

/// The `Guard` type describes types which define boundary checking strategies.
pub trait Guard {
    /// Check the size of the given byte slice against a particular type.
    ///
    /// # Errors
    ///
    /// If the slice's size does not comply with this guard, an error
    /// which specifies the incompatibility is returned.
    fn check<T>(v: &[u8]) -> Result<(), GuardError>;
}

/// Single value guard: The byte slice must have exactly enough bytes to fill a single
/// instance of a type.
pub struct SingleValueGuard;
impl Guard for SingleValueGuard {
    fn check<T>(bytes: &[u8]) -> Result<(), GuardError> {
        if bytes.len() != align_of::<T>() {
            Err(GuardError {
                required: align_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::InexactByteCount,
            })
        } else {
            Ok(())
        }
    }
}

/// Pedantic guard: The byte slice must have at least enough bytes to fill a single
/// instance of a type, and should not have extraneous data.
pub struct PedanticGuard;
impl Guard for PedanticGuard {
    fn check<T>(bytes: &[u8]) -> Result<(), GuardError> {
        if bytes.len() < align_of::<T>() {
            Err(GuardError {
                required: align_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::NotEnoughBytes,
            })
        } else if bytes.len() % align_of::<T>() != 0 {
            Err(GuardError {
                required: align_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::InexactByteCount,
            })
        } else {
            Ok(())
        }
    }
}

/// A strict guard: The byte slice should not have extraneous data, but can be
/// empty.
pub struct StrictGuard;
impl Guard for StrictGuard {
    fn check<T>(bytes: &[u8]) -> Result<(), GuardError> {
        if bytes.len() % align_of::<T>() != 0 {
            Err(GuardError {
                required: align_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::InexactByteCount,
            })
        } else {
            Ok(())
        }
    }
}

/// A basic reasonable guard: The byte slice must have at least enough bytes to fill a single
/// instance of a type, and extraneous data is ignored.
pub struct BasicGuard;
impl Guard for BasicGuard {
    fn check<T>(bytes: &[u8]) -> Result<(), GuardError> {
        if bytes.len() < align_of::<T>() {
            Err(GuardError {
                required: align_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::NotEnoughBytes,
            })
        } else {
            Ok(())
        }
    }
}

/// Permissive guard: The resulting slice would have as many instances of a type as will
/// fit, rounded down. Therefore, this guard will never yield an error.
pub struct PermissiveGuard;
impl Guard for PermissiveGuard {
    #[inline]
    fn check<T>(_bytes: &[u8]) -> Result<(), GuardError> {
        Ok(())
    }
}

//! The `guard` module exposes an API for memory boundary checking.
//!
//! # Examples:
//!
//! In order to check whether a value would fit in the given
//! slice without extraneous space:
//!
//! ```
//! # use safe_transmute::Error;
//! # use safe_transmute::guard::{SingleValueGuard, Guard};
//! # fn run() -> Result<(), Error> {
//! SingleValueGuard::check::<u32>(&[0x00, 0x01, 0x00, 0x02])?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! Different guard types implement different checking strategies.
//! For example, the pedantic guard type [`PedanticGuard`](struct.PedanticGuard.html) requires
//! the slice to have space for at least one value, and not have
//! extraneous bytes at the end.
//!
//! ```
//! # use safe_transmute::Error;
//! # use safe_transmute::guard::{PedanticGuard, Guard};
//! # fn run() -> Result<(), Error> {
//! PedanticGuard::check::<u16>(&[0xAA, 0xAA, 0xBB, 0xBB, 0xCC, 0xCC])?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! [`PermissiveGuard`](struct.PermissiveGuard.html), on the other hand, will accept any memory slice.
//!
//! ```
//! # use safe_transmute::Error;
//! # use safe_transmute::guard::{PermissiveGuard, Guard};
//! # fn run() -> Result<(), Error> {
//! PermissiveGuard::check::<i16>(b"covfefe")?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! If the check fails, the resulting [`GuardError`](../type.GuardError.html) value describes why.
//!
//! ```
//! # use safe_transmute::{GuardError, ErrorReason};
//! # use safe_transmute::guard::{PedanticGuard, Guard};
//! assert_eq!(PedanticGuard::check::<i16>(b"covfefe"),
//!            Err(GuardError {
//!                required: 2,
//!                actual: 7,
//!                reason: ErrorReason::InexactByteCount,
//!            }));
//! ```
//! 
//! Regardless of the chosen strategy, guarded transmutation functions will
//! always ensure that no out of bounds access is attempted, usually by
//! restricting the output to spatially safe portions of the input.


use error::{ErrorReason, GuardError};
use std::mem::size_of;


/// The trait describes types which define boundary checking strategies.
/// See the [module-level documentation](index.html) for more details.
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
        if bytes.len() != size_of::<T>() {
            Err(GuardError {
                required: size_of::<T>(),
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
        if bytes.len() < size_of::<T>() {
            Err(GuardError {
                required: size_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::NotEnoughBytes,
            })
        } else if bytes.len() % size_of::<T>() != 0 {
            Err(GuardError {
                required: size_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::InexactByteCount,
            })
        } else {
            Ok(())
        }
    }
}


/// An all-or-nothing guard: The byte slice should not have extraneous data, but can be
/// empty, unlike `PedanticGuard`.
pub struct AllOrNothingGuard;

impl Guard for AllOrNothingGuard {
    fn check<T>(bytes: &[u8]) -> Result<(), GuardError> {
        if bytes.len() % size_of::<T>() != 0 {
            Err(GuardError {
                required: size_of::<T>(),
                actual: bytes.len(),
                reason: ErrorReason::InexactByteCount,
            })
        } else {
            Ok(())
        }
    }
}


/// A single-or-many guard: The byte slice must have at least enough bytes to fill a single
/// instance of a type, and extraneous data is ignored.
pub struct SingleManyGuard;

impl Guard for SingleManyGuard {
    fn check<T>(bytes: &[u8]) -> Result<(), GuardError> {
        if bytes.len() < size_of::<T>() {
            Err(GuardError {
                required: size_of::<T>(),
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
    fn check<T>(_: &[u8]) -> Result<(), GuardError> {
        Ok(())
    }
}

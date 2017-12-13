use error::{GuardError, ErrorReason};
use std::mem::align_of;

/// The `Guard` type describes types which define boundary checking strategies.
pub trait Guard {
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

/// A strict guard: The byte slice should not have extraneous data, but can be empty.
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

/// Permissive guard: The resulting slice will have as many instances of a type as will
/// fit, rounded down. Therefore, this guard will never yield an error.
pub struct PermissiveGuard;
impl Guard for PermissiveGuard {
    #[inline]
    fn check<T>(_bytes: &[u8]) -> Result<(), GuardError> {
        Ok(())
    }
}

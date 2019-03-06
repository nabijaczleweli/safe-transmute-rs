//! Detectable and recoverable-from transmutation precondition errors.

#[cfg(feature = "std")]
use crate::PodTransmutable;
use core::fmt;
#[cfg(feature = "std")]
use std::error::Error as StdError;

/// A transmutation error. This type describes possible errors originating
/// from operations in this crate.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{ErrorReason, Error, transmute_bool_pedantic};
/// # unsafe {
/// assert_eq!(transmute_bool_pedantic(&[0x05]), Err(Error::InvalidValue));
/// # }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    /// The data does not respect the target type's boundaries.
    Guard(GuardError),
    /// The given data slice is not properly aligned for the target type.
    Unaligned(UnalignedError),
    /// The given data vector is not properly aligned for the target type.
    /// 
    /// Does not exist in `no_std`.
    #[cfg(feature = "std")]
    UnalignedVec(UnalignedVecError),
    /// The data contains an invalid value for the target type.
    InvalidValue,
}

#[cfg(feature = "std")]
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Guard(ref e) => e.description(),
            Error::Unaligned(ref e) => e.description(),
            Error::UnalignedVec(ref e) => e.description(),
            Error::InvalidValue => "invalid target value",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Guard(ref e) => e.fmt(f),
            Error::Unaligned(ref e) => e.fmt(f),
            #[cfg(feature = "std")]
            Error::UnalignedVec(ref e) => e.fmt(f),
            Error::InvalidValue => f.write_str("Invalid target value"),
        }
    }
}

impl From<GuardError> for Error {
    fn from(o: GuardError) -> Error {
        Error::Guard(o)
    }
}

impl From<UnalignedError> for Error {
    fn from(o: UnalignedError) -> Error {
        Error::Unaligned(o)
    }
}

#[cfg(feature = "std")]
impl From<UnalignedVecError> for Error {
    fn from(o: UnalignedVecError) -> Error {
        Error::UnalignedVec(o)
    }
}

/// A slice boundary guard error, usually created by a
/// [`Guard`](./guard/trait.Guard.html).
///
/// # Examples
///
/// ```
/// # use safe_transmute::{ErrorReason, GuardError};
/// # use safe_transmute::guard::{Guard, SingleManyGuard};
/// # unsafe {
/// assert_eq!(
///     SingleManyGuard::check::<u16>(&[0x00]),
///     Err(GuardError {
///         required: 16 / 8,
///         actual: 1,
///         reason: ErrorReason::NotEnoughBytes,
///     })
/// );
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GuardError {
    /// The required amount of bytes for transmutation.
    pub required: usize,
    /// The actual amount of bytes.
    pub actual: usize,
    /// Why this `required`/`actual`/`T` combo is an error.
    pub reason: ErrorReason,
}

/// How the type's size compares to the received byte count and the
/// transmutation function's characteristic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorReason {
    /// Too few bytes to fill even one instance of a type.
    NotEnoughBytes,
    /// Too many bytes to fill a type.
    ///
    /// Currently unused.
    TooManyBytes,
    /// The byte amount received is not the same as the type's size.
    InexactByteCount,
}

#[cfg(feature = "std")]
impl StdError for GuardError {
    fn description(&self) -> &str {
        self.reason.description()
    }
}

impl fmt::Display for GuardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (required: {}, actual: {})", self.reason.description(), self.required, self.actual)
    }
}

impl ErrorReason {
    /// Retrieve a human readable description of the reason.
    pub fn description(self) -> &'static str {
        match self {
            ErrorReason::NotEnoughBytes => "Not enough bytes to fill type",
            ErrorReason::TooManyBytes => "Too many bytes for type",
            ErrorReason::InexactByteCount => "Not exactly the amount of bytes for type",
        }
    }
}

/// Unaligned memory access error.
///
/// Returned when the given data slice is not properly aligned for the target
/// type. It would have been properly aligned if `offset` bytes were shifted
/// (discarded) from the front of the slice.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct UnalignedError {
    /// The required amount of bytes to discard at the front for the attempted
    /// transmutation to be successful.
    pub offset: usize,
}

#[cfg(feature = "std")]
impl StdError for UnalignedError {
    fn description(&self) -> &str {
        "data is unaligned"
    }
}

impl fmt::Display for UnalignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "data is unaligned (off by {} bytes)", self.offset)
    }
}

#[cfg(feature = "std")]
impl UnalignedError {
    /// Add a vector of bytes to make this an error of type
    /// `UnalignedVecError`.
    pub fn with_vec(self, vec: Vec<u8>) -> UnalignedVecError {
        UnalignedVecError {
            offset: self.offset,
            vec,
        }
    }
}

/// Unaligned vector transmutation error.
///
/// Returned when the given data vector is not properly aligned for the
/// target type. It would have been properly aligned if `offset` bytes were
/// shifted (discarded) from the front of the slice.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct UnalignedVecError {
    /// The required amount of bytes to discard at the front for the attempted
    /// transmutation to be successful.
    pub offset: usize,
    /// The original vector.
    pub vec: Vec<u8>,
}

#[cfg(feature = "std")]
impl UnalignedVecError {
    /// Create a copy of the data and transmute it. As the new vector will be
    /// properly aligned for accessing values of type `T`, this operation will
    /// never fail.
    /// 
    /// # Safety
    /// 
    /// The byte data in the vector needs to correspond to a valid contiguous
    /// sequence of `T` values.
    pub unsafe fn copy_unchecked<T>(&self) -> Vec<T> {
        let len = self.vec.len() / core::mem::size_of::<T>();
        let mut out = Vec::with_capacity(len);
        out.set_len(len);
        core::ptr::copy_nonoverlapping(self.vec.as_ptr() as *const u8, out.as_mut_ptr() as *mut u8, len * core::mem::size_of::<T>());
        out
    }

    /// Create a copy of the data and transmute it. As `T` is safely
    /// transmutable, and the new vector will be properly aligned for accessing
    /// values of type `T`, this operation is safe and will never fail.
    pub fn copy<T: PodTransmutable>(&self) -> Vec<T> {
        unsafe {
            // no value checks needed thanks to `PodTransmutable`
            self.copy_unchecked::<T>()
        }
    }
}

#[cfg(feature = "std")]
impl StdError for UnalignedVecError {
    fn description(&self) -> &str {
        "vector is unaligned"
    }
}

#[cfg(feature = "std")]
impl fmt::Display for UnalignedVecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vector is unaligned (off by {} bytes)", self.offset)
    }
}

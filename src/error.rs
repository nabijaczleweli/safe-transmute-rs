use std::error::Error as StdError;
use std::fmt;


/// A transmutation error. This type describes possible errors originating
/// from operations in this crate.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{ErrorReason, Error, guarded_transmute_bool_pedantic};
/// # unsafe {
/// assert_eq!(guarded_transmute_bool_pedantic(&[0x05]),
///            Err(Error::InvalidValue));
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    /// The data does not respect the target type's boundaries.
    Guard(GuardError),
    /// The given data slice is not properly aligned for the target type.
    /// It would have been properly aligned if `offset` bytes were shifted
    /// (discarded) from the front of the slice.
    /// 
    /// This is currently unused.
    Unaligned {
        offset: usize
    },
    /// The data contains an invalid value for the target type.
    InvalidValue,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Guard(ref e) => e.description(),
            Error::Unaligned { .. } => "Unaligned data slice",
            Error::InvalidValue => "Invalid target value",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Guard(ref e) => e.fmt(f),
            Error::Unaligned { offset } => write!(f, "{} (off by {} bytes)", self.description(), offset),
            Error::InvalidValue => write!(f, "{}", self.description()),
        }
    }
}

impl From<GuardError> for Error {
    fn from(o: GuardError) -> Error {
        Error::Guard(o)
    }
}


/// A slice boundary guard error, usually created by a [`Guard`](./guard/trait.Guard.html).
///
/// # Examples
///
/// ```
/// # use safe_transmute::{ErrorReason, GuardError};
/// # use safe_transmute::guard::{Guard, SingleManyGuard};
/// # unsafe {
/// assert_eq!(SingleManyGuard::check::<u16>(&[0x00]),
///            Err(GuardError {
///                required: 16 / 8,
///                actual: 1,
///                reason: ErrorReason::NotEnoughBytes,
///            }));
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

/// How the type's size compares to the received byte count and the transmutation function's characteristic.
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


impl StdError for GuardError {
    fn description(&self) -> &str {
        match self.reason {
            ErrorReason::NotEnoughBytes => "Not enough bytes to fill type",
            ErrorReason::TooManyBytes => "Too many bytes for type",
            ErrorReason::InexactByteCount => "Not exactly the amount of bytes for type",
        }
    }
}

impl fmt::Display for GuardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (required: {}, actual: {})", self.description(), self.required, self.actual)
    }
}

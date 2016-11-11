use std::error::Error as StdError;
use std::fmt;


/// A transmutation error.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{ErrorReason, Error, guarded_transmute};
/// # unsafe {
/// assert_eq!(guarded_transmute::<u16>(&[0x00]),
///            Err(Error {
///                required: 16 / 8,
///                actual: 1,
///                reason: ErrorReason::NotEnoughBytes,
///            }));
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error {
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


impl StdError for Error {
    fn description(&self) -> &str {
        match self.reason {
            ErrorReason::NotEnoughBytes => "Not enough bytes to fill type",
            ErrorReason::TooManyBytes => "Too many bytes for type",
            ErrorReason::InexactByteCount => "Not exactly the amount of bytes for type",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (required: {}, actual: {})", self.description(), self.required, self.actual)
    }
}

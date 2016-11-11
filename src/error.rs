use std::error::Error as StdError;
use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error {
    pub required: usize,
    pub actual: usize,
    pub reason: ErrorReason,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorReason {
    NotEnoughBytes,
    TooManyBytes,
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

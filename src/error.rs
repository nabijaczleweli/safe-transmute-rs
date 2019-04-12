//! Detectable and recoverable-from transmutation precondition errors.


#[cfg(feature = "std")]
use std::error::Error as StdError;
#[cfg(feature = "std")]
use core::mem::{size_of, align_of};
use core::fmt;
#[cfg(feature = "std")]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use super::trivial::TriviallyTransmutable;

/// A transmutation error. This type describes possible errors originating
/// from operations in this crate. The two type parameters represent the
/// source element type and the target element type respectively.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{ErrorReason, Error, transmute_bool_pedantic};
/// assert_eq!(transmute_bool_pedantic(&[0x05]), Err(Error::InvalidValue));
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Error<S, T> {
    /// The data does not respect the target type's boundaries.
    Guard(GuardError),
    /// The given data slice is not properly aligned for the target type.
    Unaligned(UnalignedError),
    /// The data vector's element type does not have the same size and minimum
    /// alignment as the target type.
    ///
    /// Does not exist in `no_std`.
    #[cfg(feature = "std")]
    IncompatibleVecTarget(IncompatibleVecTargetError<S, T>),
    /// The data contains an invalid value for the target type.
    InvalidValue,

    #[cfg(not(feature = "std"))]
    #[doc(hidden)]
    /// Do not use this!
    None(::core::marker::PhantomData<(S, T)>),
}

impl<S, T> fmt::Debug for Error<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Guard(e) => write!(f, "Guard({:?})", e),
            Error::Unaligned(e) => write!(f, "Unaligned({:?})", e),
            Error::InvalidValue => f.write_str("InvalidValue"),
            #[cfg(feature = "std")]
            Error::IncompatibleVecTarget(_) => f.write_str("IncompatibleVecTarget"),
            #[cfg(not(feature = "std"))]
            Error::None(_) => f.write_str("None"),
        }
    }
}

#[cfg(feature = "std")]
impl<S, T> StdError for Error<S, T> {
    fn description(&self) -> &str {
        match self {
            Error::Guard(e) => e.description(),
            Error::Unaligned(e) => e.description(),
            Error::InvalidValue => "invalid target value",
            Error::IncompatibleVecTarget(e) => e.description(),
            #[cfg(not(feature = "std"))]
            Error::None(_) => "None (dummy error, do not use!)",
        }
    }
}

impl<S, T> fmt::Display for Error<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Guard(e) => e.fmt(f),
            Error::Unaligned(e) => e.fmt(f),
            Error::InvalidValue => f.write_str("Invalid target value"),
            #[cfg(feature = "std")]
            Error::IncompatibleVecTarget(e) => e.fmt(f),
            #[cfg(not(feature = "std"))]
            Error::None(_) => f.write_str("None"),
        }
    }
}

impl<S, T> From<GuardError> for Error<S, T> {
    fn from(o: GuardError) -> Self {
        Error::Guard(o)
    }
}

impl<S, T> From<UnalignedError> for Error<S, T> {
    fn from(o: UnalignedError) -> Self {
        Error::Unaligned(o)
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

/// Incompatible vector transmutation error.
///
/// Returned when the element type `T` does not allow a safe vector
/// transmutation to the target element type `U`. This happens when either
/// the size or minimum memory alignment requirements are not met:
/// 
/// - `std::mem::align_of::<T>() != std::mem::align_of::<U>()` 
/// - `std::mem::size_of::<T>() != std::mem::size_of::<U>()`
#[cfg(feature = "std")]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct IncompatibleVecTargetError<S, T> {
    /// The original vector.
    pub vec: Vec<S>,
    /// The target element type
    target: PhantomData<T>,
}

#[cfg(feature = "std")]
impl<S, T> IncompatibleVecTargetError<S, T> {
    /// Create an error with the given vector.
    pub fn new(vec: Vec<S>) -> Self {
        IncompatibleVecTargetError {
            vec,
            target: PhantomData,
        }
    }

    /// Create a copy of the data and transmute it. As the new vector will be
    /// properly aligned for accessing values of type `U`, this operation will
    /// never fail.
    ///
    /// # Safety
    ///
    /// The byte data in the vector needs to correspond to a valid contiguous
    /// sequence of `U` values.
    pub unsafe fn copy_unchecked(&self) -> Vec<T> {
        let len = self.vec.len() * core::mem::size_of::<S>() / core::mem::size_of::<T>();
        let mut out = Vec::with_capacity(len);
        core::ptr::copy_nonoverlapping(self.vec.as_ptr() as *const u8, out.as_mut_ptr() as *mut u8, len * core::mem::size_of::<T>());
        out.set_len(len);
        out
    }

    /// Create a copy of the data and transmute it. As `S` is trivially
    /// transmutable, and the new vector will be properly allocated for accessing
    /// values of type `U`, this operation is safe and will never fail.
    pub fn copy(&self) -> Vec<T>
    where
        T: TriviallyTransmutable,
    {
        unsafe {
            // no value checks needed thanks to `TriviallyTransmutable`
            self.copy_unchecked()
        }
    }
}

#[cfg(feature = "std")]
impl<S, T> From<IncompatibleVecTargetError<S, T>> for Error<S, T> {
    fn from(e: IncompatibleVecTargetError<S, T>) -> Self {
        Error::IncompatibleVecTarget(e)
    }
} 

#[cfg(feature = "std")]
impl<S, T> fmt::Debug for IncompatibleVecTargetError<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IncompatibleVecTargetError")
            .field("size_of<S>", &size_of::<S>())
            .field("align_of<S>", &align_of::<S>())
            .field("size_of<T>", &size_of::<T>())
            .field("align_of<T>", &align_of::<T>())
            .finish()
    }
}

#[cfg(feature = "std")]
impl<S, T> StdError for IncompatibleVecTargetError<S, T> {
    fn description(&self) -> &str {
        "incompatible target type"
    }
}

#[cfg(feature = "std")]
impl<S, T> fmt::Display for IncompatibleVecTargetError<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "incompatible target type (size: {}, align: {}) for transmutation from source (size: {}, align: {})",
            size_of::<T>(),
            align_of::<T>(),
            size_of::<S>(),
            align_of::<S>()
        )
    }
}

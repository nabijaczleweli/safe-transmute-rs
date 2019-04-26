//! Detectable and recoverable-from transmutation precondition errors.


use core::fmt;
#[cfg(feature = "std")]
use core::ptr;
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::error::Error as StdError;
#[cfg(feature = "std")]
use core::mem::{align_of, size_of};
#[cfg(feature = "std")]
use self::super::trivial::TriviallyTransmutable;


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
pub enum Error<'a, S, T> {
    /// The data does not respect the target type's boundaries.
    Guard(GuardError),
    /// The given data slice is not properly aligned for the target type.
    Unaligned(UnalignedError<'a, S, T>),
    /// The data vector's element type does not have the same size and minimum
    /// alignment as the target type.
    ///
    /// Does not exist in `no_std`.
    #[cfg(feature = "std")]
    IncompatibleVecTarget(IncompatibleVecTargetError<S, T>),
    /// The data contains an invalid value for the target type.
    InvalidValue,
}

impl<'a, S, T> fmt::Debug for Error<'a, S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Guard(e) => write!(f, "Guard({:?})", e),
            Error::Unaligned(e) => write!(f, "Unaligned({:?})", e),
            Error::InvalidValue => f.write_str("InvalidValue"),
            #[cfg(feature = "std")]
            Error::IncompatibleVecTarget(_) => f.write_str("IncompatibleVecTarget"),
        }
    }
}

#[cfg(feature = "std")]
impl<'a, S, T> StdError for Error<'a, S, T> {
    fn description(&self) -> &str {
        match self {
            Error::Guard(e) => e.description(),
            Error::Unaligned(e) => e.description(),
            Error::InvalidValue => "invalid target value",
            Error::IncompatibleVecTarget(e) => e.description(),
        }
    }
}

impl<'a, S, T> fmt::Display for Error<'a, S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Guard(e) => e.fmt(f),
            Error::Unaligned(e) => e.fmt(f),
            Error::InvalidValue => f.write_str("Invalid target value"),
            #[cfg(feature = "std")]
            Error::IncompatibleVecTarget(e) => e.fmt(f),
        }
    }
}

impl<'a, S, T> From<GuardError> for Error<'a, S, T> {
    fn from(o: GuardError) -> Self {
        Error::Guard(o)
    }
}

impl<'a, S, T> From<UnalignedError<'a, S, T>> for Error<'a, S, T> {
    fn from(o: UnalignedError<'a, S, T>) -> Self {
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

/// Create a copy of the given data, transmuted into a vector.
///
/// # Safety
///
/// The byte data in the vector needs to correspond to a valid contiguous
/// sequence of `T` values.
#[cfg(feature = "std")]
unsafe fn copy_to_vec_unchecked<S, T>(data: &[S]) -> Vec<T> {
    let len = data.len() * size_of::<S>() / size_of::<T>();

    let mut out = Vec::with_capacity(len);
    ptr::copy_nonoverlapping(data.as_ptr() as *const u8, out.as_mut_ptr() as *mut u8, len * size_of::<T>());

    out.set_len(len);
    out
}

/// Unaligned memory access error.
///
/// Returned when the given data slice is not properly aligned for the target
/// type. It would have been properly aligned if `offset` bytes were shifted
/// (discarded) from the front of the slice.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct UnalignedError<'a, S, T> {
    /// The required amount of bytes to discard at the front for the attempted
    /// transmutation to be successful.
    pub offset: usize,
    /// A slice of the original source data.
    pub source: &'a [S],

    phantom: PhantomData<T>,
}

impl<'a, S, T> UnalignedError<'a, S, T> {
    pub fn new(offset: usize, source: &'a [S]) -> Self {
        UnalignedError {
            offset: offset,
            source: source,
            phantom: PhantomData,
        }
    }

    /// Create a copy of the source data, transmuted into a vector. As the
    /// vector will be properly aligned for accessing values of type `T`, this
    /// operation will not fail due to memory alignment constraints.
    ///
    /// # Safety
    ///
    /// The byte data in the slice needs to correspond to a valid contiguous
    /// sequence of `T` values.
    #[cfg(feature = "std")]
    pub unsafe fn copy_unchecked(&self) -> Vec<T> {
        copy_to_vec_unchecked::<S, T>(self.source)
    }

    /// Create a copy of the source data, transmuted into a vector. As `S` is
    /// trivially transmutable, and the vector will be properly allocated
    /// for accessing values of type `T`, this operation is safe and will never
    /// fail.
    #[cfg(feature = "std")]
    pub fn copy(&self) -> Vec<T>
        where S: TriviallyTransmutable
    {
        unsafe {
            // no value checks needed thanks to `TriviallyTransmutable`
            self.copy_unchecked()
        }
    }
}

impl<'a, S, T> fmt::Debug for UnalignedError<'a, S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Summarize the output of the source slice to just its
        // length, so that it does not require `S: Debug`.
        struct Source {
            len: usize,
        }

        impl fmt::Debug for Source {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("&[S]")
                    .field("len", &self.len)
                    .finish()
            }
        }

        f.debug_struct("UnalignedError")
            .field("offset", &self.offset)
            .field("source", &Source { len: self.source.len() })
            .finish()
    }
}

#[cfg(feature = "std")]
impl<'a, S, T> StdError for UnalignedError<'a, S, T> {
    fn description(&self) -> &str {
        "data is unaligned"
    }
}

impl<'a, S, T> fmt::Display for UnalignedError<'a, S, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "data is unaligned (off by {} bytes)", self.offset)
    }
}


/// Incompatible vector transmutation error.
///
/// Returned when the element type `S` does not allow a safe vector
/// transmutation to the target element type `T`. This happens when either
/// the size or minimum memory alignment requirements are not met:
///
/// - `std::mem::align_of::<S>() != std::mem::align_of::<T>()`
/// - `std::mem::size_of::<S>() != std::mem::size_of::<T>()`
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
            vec: vec,
            target: PhantomData,
        }
    }

    /// Create a copy of the data, transmuted into a new vector. As the vector
    /// will be properly aligned for accessing values of type `T`, this
    /// operation will not fail due to memory alignment constraints.
    ///
    /// # Safety
    ///
    /// The byte data in the vector needs to correspond to a valid contiguous
    /// sequence of `T` values.
    pub unsafe fn copy_unchecked(&self) -> Vec<T> {
        copy_to_vec_unchecked::<S, T>(&self.vec)
    }

    /// Create a copy of the data, transmuted into a new vector. As `S` is
    /// trivially transmutable, and the new vector will be properly allocated
    /// for accessing values of type `T`, this operation is safe and will never fail.
    pub fn copy(&self) -> Vec<T>
        where S: TriviallyTransmutable
    {
        unsafe {
            // no value checks needed thanks to `TriviallyTransmutable`
            self.copy_unchecked()
        }
    }
}

#[cfg(feature = "std")]
impl<'a, S, T> From<IncompatibleVecTargetError<S, T>> for Error<'a, S, T> {
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
        write!(f,
               "incompatible target type (size: {}, align: {}) for transmutation from source (size: {}, align: {})",
               size_of::<T>(),
               align_of::<T>(),
               size_of::<S>(),
               align_of::<S>())
    }
}

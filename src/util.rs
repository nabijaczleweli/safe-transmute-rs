//! Module containing various utility functions.


/// Retrieve the result of a transmutation, copying the data if this cannot be
/// done safely due to memory alignment constraints.
/// 
/// The macro, not unlike `try!`, will short-circuit certain errors with
/// `return`, namely guard condition and invalid value errors. When the operation
/// fails due to either an unaligned transmutation or an incompatible vector
/// element transmutation target, the transmutation is once again attempted by
/// copying (as in a `memcpy`) the output into a vector.
/// 
/// This expands into a single expression of type `Cow<[T]>`. where `T` is the
/// target type.
/// 
/// 
/// # Example
/// 
/// ```
/// # #![cfg(feature = "std")]
/// # #[macro_use]
/// # extern crate safe_transmute;
/// # use safe_transmute::{transmute_many, SingleManyGuard};
/// # fn run() -> Result<(), Box<::std::error::Error>> {
/// let bytes = &[0x00, 0x01, 0x12, 0x34, 0x00]; // 1 byte unused
/// let hwords = try_copy!(transmute_many::<u16, SingleManyGuard>(bytes));
/// assert_eq!(
///     &*hwords,
///     &[
///         u16::from_be(0x0001),
///         u16::from_be(0x1234),
///     ][..]);
/// # Ok(())
/// # }
/// # fn main() {
/// # run().unwrap()
/// # }
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! try_copy {
    ($res: expr) => {{
        use std::borrow::Cow;

        $res.map_err($crate::Error::from)
            .map(Cow::from)
            .or_else(|e| e.copy().map(Cow::Owned))?
    }};
}

/// Retrieve the result of a transmutation, copying the data if this cannot be
/// done safely due to memory alignment constraints. It is equivalent to the
/// macro [`try_copy!`](macro.try_copy.html), except that it does not check
/// whether the target type is trivially transmutable.
/// 
/// # Safety
/// 
/// The source data needs to correspond to a valid contiguous sequence of
/// `T` values.
/// 
/// # Example
/// 
/// ```
/// # #![cfg(feature = "std")]
/// # #[macro_use]
/// # extern crate safe_transmute;
/// # use safe_transmute::{transmute_many, SingleManyGuard};
/// # fn run() -> Result<(), Box<::std::error::Error>> {
/// let bytes = &[0x00, 0x01, 0x12, 0x34, 0x00]; // 1 byte unused
/// unsafe {
///     let hwords = try_copy_unchecked!(transmute_many::<u16, SingleManyGuard>(bytes));
///     assert_eq!(
///         &*hwords,
///         &[
///             u16::from_be(0x0001),
///             u16::from_be(0x1234),
///         ][..]);
/// }
/// # Ok(())
/// # }
/// # fn main() {
/// # run().unwrap()
/// # }
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! try_copy_unchecked {
    ($res: expr) => {{
        use std::borrow::Cow;
        
        $res.map_err($crate::Error::from)
            .map(Cow::from)
            .or_else(|e| e.copy_unchecked().map(Cow::Owned))?
    }};
}

/// If the specified 32-bit float is a signaling NaN, make it a quiet NaN.
///
/// Based on an old version of
/// [`f32::from_bits()`](https://github.com/rust-lang/rust/pull/39271/files#diff-f60977ab00fd9ea9ba7ac918e12a8f42R1279).
pub fn designalise_f32(f: f32) -> f32 {
    from_bits_f32_designalised(f.to_bits())
}

/// If the specified 64-bit float is a signaling NaN, make it a quiet NaN.
///
/// Based on an old version of
/// [`f64::from_bits()`](https://github.com/rust-lang/rust/pull/39271/files#diff-2ae382eb5bbc830a6b884b8a6ba5d95fR1171).
pub fn designalise_f64(f: f64) -> f64 {
    from_bits_f64_designalised(f.to_bits())
}

/// Reinterpret the given bits as a 32-bit float. If the specified word is a
/// signaling NaN once interpreted, make it a quiet NaN.
pub fn from_bits_f32_designalised(mut bits: u32) -> f32 {
    const EXP_MASK: u32 = 0x7F80_0000;
    const QNAN_MASK: u32 = 0x0040_0000;
    const FRACT_MASK: u32 = 0x007F_FFFF;

    if bits & EXP_MASK == EXP_MASK && bits & FRACT_MASK != 0 {
        // If we have a NaN value, we
        // convert signaling NaN values to quiet NaN
        // by setting the the highest bit of the fraction
        bits |= QNAN_MASK;
    }

    f32::from_bits(bits)
}

/// Reinterpret the given bits as a 64-bit float. If the specified word is a
/// signaling NaN once interpreted, make it a quiet NaN.
pub fn from_bits_f64_designalised(mut bits: u64) -> f64 {
    const EXP_MASK: u64 = 0x7FF0_0000_0000_0000;
    const QNAN_MASK: u64 = 0x0001_0000_0000_0000;
    const FRACT_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;

    if bits & EXP_MASK == EXP_MASK && bits & FRACT_MASK != 0 {
        // If we have a NaN value, we
        // convert signaling NaN values to quiet NaN
        // by setting the the highest bit of the fraction
        bits |= QNAN_MASK;
    }

    f64::from_bits(bits)
}

//! Module containing various utility functions.


use core::mem::transmute;


/// If the specified 32-bit float is a signaling NaN, make it a quiet NaN.
///
/// Based on an old version of
/// [`f32::from_bits()`](https://github.com/rust-lang/rust/pull/39271/files#diff-f60977ab00fd9ea9ba7ac918e12a8f42R1279)
pub fn designalise_f32(f: f32) -> f32 {
    const EXP_MASK: u32 = 0x7F80_0000;
    const QNAN_MASK: u32 = 0x0040_0000;
    const FRACT_MASK: u32 = 0x007F_FFFF;

    let mut f: u32 = unsafe { transmute(f) };

    if f & EXP_MASK == EXP_MASK && f & FRACT_MASK != 0 {
        // If we have a NaN value, we
        // convert signaling NaN values to quiet NaN
        // by setting the the highest bit of the fraction
        f |= QNAN_MASK;
    }

    f32::from_bits(f)
}

/// If the specified 64-bit float is a signaling NaN, make it a quiet NaN.
///
/// Based on an old version of
/// [`f64::from_bits()`](https://github.com/rust-lang/rust/pull/39271/files#diff-2ae382eb5bbc830a6b884b8a6ba5d95fR1171)
pub fn designalise_f64(f: f64) -> f64 {
    const EXP_MASK: u64 = 0x7FF0_0000_0000_0000;
    const QNAN_MASK: u64 = 0x0001_0000_0000_0000;
    const FRACT_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;

    let mut f: u64 = unsafe { transmute(f) };

    if f & EXP_MASK == EXP_MASK && f & FRACT_MASK != 0 {
        // If we have a NaN value, we
        // convert signaling NaN values to quiet NaN
        // by setting the the highest bit of the fraction
        f |= QNAN_MASK;
    }

    f64::from_bits(f)
}

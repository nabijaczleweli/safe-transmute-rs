//! Module containing various utility functions.


use core::mem::{transmute, align_of, size_of};
use super::Error;


/// If the specified 32-bit float is a signaling NaN, make it a quiet NaN.
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

/// Check whether the given data slice of `T`s is properly aligned for reading
/// and writing as a slice of `U`s.
///
/// # Errors
///
/// An `Error::Unaligned` error is raised with the number of bytes to discard
/// from the front in order to make the conversion safe from alignment concerns.
pub fn check_alignment<T, U>(data: &[T]) -> Result<(), Error> {
    // TODO this could probably become more efficient once `ptr::align_offset`
    // is stabilized (#44488)
    let ptr = data.as_ptr();
    let offset = ptr as usize % align_of::<U>();
    if offset > 0 {
        // reverse the offset (from "bytes to insert" to "bytes to remove")
        Err(Error::Unaligned { offset: size_of::<U>() - offset })
    } else {
        Ok(())
    }
}

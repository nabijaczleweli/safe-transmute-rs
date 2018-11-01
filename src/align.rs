//! Alignment checking primitives.


use core::mem::{align_of, size_of};
use self::super::error::UnalignedError;


/// Check whether the given data slice of `T`s is properly aligned for reading
/// and writing as a slice of `U`s.
///
/// # Errors
///
/// An `Error::Unaligned` error is returned with the number of bytes to discard
/// from the front in order to make the conversion safe from alignment concerns.
pub fn check_alignment<T, U>(data: &[T]) -> Result<(), UnalignedError> {
    // TODO this could probably become more efficient once `ptr::align_offset`
    // is stabilized (#44488)
    let ptr = data.as_ptr();
    let offset = ptr as usize % align_of::<U>();
    if offset > 0 {
        // reverse the offset (from "bytes to insert" to "bytes to remove")
        Err(UnalignedError { offset: size_of::<U>() - offset })
    } else {
        Ok(())
    }
}

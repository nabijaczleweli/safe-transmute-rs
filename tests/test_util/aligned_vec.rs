/// Create a new vector that contains the given bytes and is sure to have a
/// memory alignment compatible with `T` at creation time.
///
/// Do not modify the vector, or this assurance is gone.
/// 
/// # Examples
/// 
/// ```
/// let data: &[u8] = &[0xFF, 0xFF, 0x03, 0x00];
/// let vev = aligned_vec::<u32>(data);
/// // the vector's data is guaranteed to be aligned for access as a u32
/// assert_eq!((vec.as_ptr() as usize) % align_of::<u32>(), 0);
/// ```
#[cfg(feature = "std")]
fn aligned_vec<T>(bytes: &[u8]) -> Vec<u8> {
    use core::mem::{align_of, forget, size_of};

    let vec_len_offset = bytes.len() % size_of::<T>();
    let vec_len = bytes.len() / size_of::<T>();
    let capacity = if vec_len_offset > 0 { vec_len + 1 } else { vec_len };
    unsafe {
        let mut v: Vec<T> = Vec::with_capacity(capacity);
        let ptr = v.as_mut_ptr() as *mut u8;
        bytes.as_ptr().copy_to_nonoverlapping(ptr, bytes.len());
        forget(v);
        let vec = Vec::from_raw_parts(ptr, bytes.len(), capacity * size_of::<T>());
        assert_eq!((vec.as_ptr() as usize) % align_of::<T>(), 0);
        assert_eq!(vec.len(), bytes.len());
        assert!(vec.capacity() >= vec.len());
        vec
    }
}

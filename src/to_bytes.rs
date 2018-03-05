//! Functions for transmutation *from* a concrete type *to* bytes.


use self::super::PodTransmutable;
use std::mem::size_of;
use std::slice;


/// Transmute a single instance of an arbitrary type into a slice of its bytes.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::guarded_transmute_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// # unsafe {
/// assert_eq!(guarded_transmute_to_bytes(&0x01234567),
/// # /*
///            &[0x67, 0x45, 0x23, 0x01]);
/// # */
/// #           [0x67, 0x45, 0x23, 0x01].le_to_native::<u32>());
/// # }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::guarded_transmute_to_bytes;
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// # unsafe {
/// assert_eq!(guarded_transmute_to_bytes(&Gene {
///                x1: 0x42,
///                x2: 0x69,
///            }),
///            &[0x42, 0x69]);
/// # }
/// ```
pub unsafe fn guarded_transmute_to_bytes<T>(from: &T) -> &[u8] {
    slice::from_raw_parts(from as *const T as *const u8, size_of::<T>())
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::guarded_transmute_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// # unsafe {
/// assert_eq!(guarded_transmute_to_bytes(&[0x0123u16, 0x4567u16]),
/// # /*
///            &[0x23, 0x01, 0x67, 0x45]);
/// # */
/// #           [0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// # }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::{PodTransmutable, guarded_transmute_to_bytes_many};
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// # unsafe {
/// assert_eq!(guarded_transmute_to_bytes_many(&[Gene {
///                                                  x1: 0x42,
///                                                  x2: 0x69,
///                                              },
///                                              Gene {
///                                                  x1: 0x12,
///                                                  x2: 0x48,
///                                              }]),
///            &[0x42, 0x69, 0x12, 0x48]);
/// # }
/// ```
pub unsafe fn guarded_transmute_to_bytes_many<T>(from: &[T]) -> &[u8] {
    slice::from_raw_parts(from.as_ptr() as *const u8, from.len() * size_of::<T>())
}

/// Transmute a single instance of a POD type into a slice of its bytes.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::guarded_transmute_to_bytes_pod;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(guarded_transmute_to_bytes_pod(&0x01234567),
/// # /*
///            &[0x67, 0x45, 0x23, 0x01]);
/// # */
/// #           [0x67, 0x45, 0x23, 0x01].le_to_native::<u32>());
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::{PodTransmutable, guarded_transmute_to_bytes_pod};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl PodTransmutable for Gene {}
///
/// assert_eq!(guarded_transmute_to_bytes_pod(&Gene {
///                x1: 0x42,
///                x2: 0x69,
///            }),
///            &[0x42, 0x69]);
/// ```
pub fn guarded_transmute_to_bytes_pod<T: PodTransmutable>(from: &T) -> &[u8] {
    unsafe { guarded_transmute_to_bytes(from) }
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::guarded_transmute_to_bytes_pod_many;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(guarded_transmute_to_bytes_pod_many(&[0x0123u16, 0x4567u16]),
/// # /*
///            &[0x23, 0x01, 0x67, 0x45]);
/// # */
/// #           [0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::{PodTransmutable, guarded_transmute_to_bytes_pod_many};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl PodTransmutable for Gene {}
///
/// assert_eq!(guarded_transmute_to_bytes_pod_many(&[Gene {
///                                                      x1: 0x42,
///                                                      x2: 0x69,
///                                                  },
///                                                  Gene {
///                                                      x1: 0x12,
///                                                      x2: 0x48,
///                                                  }]),
///            &[0x42, 0x69, 0x12, 0x48]);
/// ```
pub fn guarded_transmute_to_bytes_pod_many<T: PodTransmutable>(from: &[T]) -> &[u8] {
    unsafe { guarded_transmute_to_bytes_many(from) }
}

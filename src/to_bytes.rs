//! Functions for transmutation *from* a concrete type *to* bytes.


use self::super::TriviallyTransmutable;
#[cfg(feature = "std")]
use self::super::Error;
use core::mem::size_of;
use core::slice;


/// Transmute a single instance of an arbitrary type into a slice of its bytes.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_unchecked;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// unsafe {
///     assert_eq!(transmute_to_bytes_unchecked(&0x0123_4567),
/// # /*
///                &[0x67, 0x45, 0x23, 0x01]);
/// # */
/// #               [0x67, 0x45, 0x23, 0x01].le_to_native::<u32>());
/// }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_unchecked;
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// unsafe {
///     assert_eq!(transmute_to_bytes_unchecked(&Gene {
///                    x1: 0x42,
///                    x2: 0x69,
///                }),
///                &[0x42, 0x69]);
/// }
/// ```
pub unsafe fn transmute_to_bytes_unchecked<S>(from: &S) -> &[u8] {
    slice::from_raw_parts(from as *const S as *const u8, size_of::<S>())
}

/// Transmute a single mutable instance of an arbitrary type into a mutable
/// slice of its bytes.
///
/// # Safety
/// 
/// This function is very ill advised, since it can be exploited to break
/// invariants of the source type. Any modification that leaves the data
/// in an inconsistent state with respect to `S` is undefined behavior.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_unchecked_mut;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// unsafe {
///     assert_eq!(transmute_to_bytes_unchecked_mut(&mut 0x0123_4567),
/// # /*
///                &mut [0x67, 0x45, 0x23, 0x01]);
/// # */
/// #              &mut [0x67, 0x45, 0x23, 0x01].le_to_native::<u32>());
/// }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_unchecked_mut;
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// unsafe {
///     assert_eq!(transmute_to_bytes_unchecked_mut(&mut Gene {
///                    x1: 0x42,
///                    x2: 0x69,
///                }),
///                &mut [0x42, 0x69]);
/// }
/// ```
pub unsafe fn transmute_to_bytes_unchecked_mut<S>(from: &mut S) -> &mut [u8] {
    slice::from_raw_parts_mut(from as *mut S as *mut u8, size_of::<S>())
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// unsafe {
///     assert_eq!(transmute_to_bytes(&[0x0123u16, 0x4567u16]),
/// # /*
///                &[0x23, 0x01, 0x67, 0x45]);
/// # */
/// #               [0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_many_unchecked;
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// unsafe {
///     assert_eq!(transmute_to_bytes_many_unchecked(&[Gene {
///                                                        x1: 0x42,
///                                                        x2: 0x69,
///                                                    },
///                                                    Gene {
///                                                        x1: 0x12,
///                                                        x2: 0x48,
///                                                    }]),
///                &[0x42, 0x69, 0x12, 0x48]);
/// }
/// ```
pub unsafe fn transmute_to_bytes_many_unchecked<S>(from: &[S]) -> &[u8] {
    slice::from_raw_parts(from.as_ptr() as *const u8, from.len() * size_of::<S>())
}

/// Transmute a mutable slice of arbitrary types into a mutable slice of their
/// bytes.
/// 
/// # Safety
/// 
/// This function is very ill advised, since it can be exploited to break
/// invariants of the source type. Any modification that leaves the data
/// in an inconsistent state with respect to `S` is undefined behavior.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_mut;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// unsafe {
///     assert_eq!(transmute_to_bytes_mut(&mut [0x0123u16, 0x4567u16]),
/// # /*
///                &[0x23, 0x01, 0x67, 0x45]);
/// # */
/// #               [0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_many_unchecked;
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// unsafe {
///     assert_eq!(transmute_to_bytes_many_unchecked(&mut [Gene {
///                                                        x1: 0x42,
///                                                        x2: 0x69,
///                                                    },
///                                                    Gene {
///                                                        x1: 0x12,
///                                                        x2: 0x48,
///                                                    }]),
///                &[0x42, 0x69, 0x12, 0x48]);
/// }
/// ```
pub unsafe fn transmute_to_bytes_many_unchecked_mut<S>(from: &mut [S]) -> &mut [u8] {
    slice::from_raw_parts_mut(from.as_mut_ptr() as *mut u8, from.len() * size_of::<S>())
}

/// Transmute a single instance of a trivially transmutable type into a slice
/// of its bytes.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::transmute_one_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(transmute_one_to_bytes(&0x0123_4567),
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
/// # use safe_transmute::{TriviallyTransmutable, transmute_one_to_bytes};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl TriviallyTransmutable for Gene {}
///
/// assert_eq!(transmute_one_to_bytes(&Gene {
///                x1: 0x42,
///                x2: 0x69,
///            }),
///            &[0x42, 0x69]);
/// ```
pub fn transmute_one_to_bytes<S: TriviallyTransmutable>(from: &S) -> &[u8] {
    unsafe { transmute_to_bytes_unchecked(from) }
}

/// Transmute a single instance of a trivially transmutable type into a slice
/// of its bytes.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::transmute_one_to_bytes_mut;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(transmute_one_to_bytes_mut(&mut 0x0123_4567),
/// # /*
///            &mut [0x67, 0x45, 0x23, 0x01]);
/// # */
/// #          &mut [0x67, 0x45, 0x23, 0x01].le_to_native::<u32>());
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::{TriviallyTransmutable, transmute_one_to_bytes_mut};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl TriviallyTransmutable for Gene {}
///
/// assert_eq!(transmute_one_to_bytes_mut(&mut Gene {
///                x1: 0x42,
///                x2: 0x69,
///            }),
///            &mut [0x42, 0x69]);
/// ```
pub fn transmute_one_to_bytes_mut<S: TriviallyTransmutable>(from: &mut S) -> &mut [u8] {
    unsafe { transmute_to_bytes_unchecked_mut(from) }
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::transmute_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(transmute_to_bytes(&[0x0123u16, 0x4567u16]),
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
/// # use safe_transmute::{TriviallyTransmutable, transmute_to_bytes};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl TriviallyTransmutable for Gene {}
///
/// assert_eq!(transmute_to_bytes(&[Gene {
///                                          x1: 0x42,
///                                          x2: 0x69,
///                                      },
///                                      Gene {
///                                          x1: 0x12,
///                                          x2: 0x48,
///                                      }]),
///            &[0x42, 0x69, 0x12, 0x48]);
/// ```
pub fn transmute_to_bytes<S: TriviallyTransmutable>(from: &[S]) -> &[u8] {
    unsafe { transmute_to_bytes_many_unchecked(from) }
}

/// Transmute a mutable slice of a trivially transmutable type into a mutable
/// slice of its bytes.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::transmute_to_bytes_mut;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(transmute_to_bytes_mut(&mut [0x0123u16, 0x4567u16]),
/// # /*
///            &mut [0x23, 0x01, 0x67, 0x45]);
/// # */
/// #          &mut [0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::{TriviallyTransmutable, transmute_to_bytes_mut};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl TriviallyTransmutable for Gene {}
///
/// assert_eq!(transmute_to_bytes_mut(&mut [Gene {
///                                             x1: 0x42,
///                                             x2: 0x69,
///                                         },
///                                         Gene {
///                                             x1: 0x12,
///                                             x2: 0x48,
///                                         }]),
///            &mut [0x42, 0x69, 0x12, 0x48]);
/// ```
pub fn transmute_to_bytes_mut<S: TriviallyTransmutable>(from: &mut [S]) -> &mut [u8] {
    unsafe { transmute_to_bytes_many_unchecked_mut(from) }
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
#[deprecated(since = "0.11.0", note = "use `transmute_to_bytes()` instead")]
pub fn guarded_transmute_to_bytes_pod_many<S: TriviallyTransmutable>(from: &[S]) -> &[u8] {
    transmute_to_bytes(from)
}

/// Transmute a vector of elements of an arbitrary type into a vector of their
/// bytes, using the same memory buffer as the former.
///
/// This is equivalent to calling [`full::transmute_vec`](../full/fn.transmute_vec.html) where
/// the target type is `u8`.
///
/// # Errors
///
/// An error is returned if the minimum memory alignment requirements are not
/// the same between `S` and `u8`:
///
/// ```
/// # /*
/// std::mem::align_of::<S>() != 1
/// # */
/// ```
///
/// The only truly safe way of doing this is to create a transmuted slice
/// view of the vector or make a copy anyway.
///
#[cfg(feature = "std")]
pub fn transmute_to_bytes_vec<S: TriviallyTransmutable>(from: Vec<S>) -> Result<Vec<u8>, Error<'static, S, u8>> {
    super::full::transmute_vec::<S, u8>(from)
}

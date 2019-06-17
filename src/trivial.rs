//! Transmutation of trivial objects
//!
//! Functions in this module are guarded from out-of-bounds memory access and
//! from unsafe transmutation target types through the use of the
//! [`TriviallyTransmutable`](trait.TriviallyTransmutable.html)) trait.
//!
//! If a certain type can be safely constructed out of any byte combination,
//! then it may implement this trait. This is the case for primitive integer
//! types (e.g. `i32`, `u32`, `i64`), arrays of other trivially transmutable
//! types, and `repr(C)` structs composed of trivially transmutable values.
//!
//! However, they are still not entirely safe because the source data may not
//! be correctly aligned for reading and writing a value of the target type.
//! The effects of this range from less performance (e.g. x86) to trapping or
//! address flooring (e.g. ARM), but this is undefined behavior nonetheless.


use self::super::guard::{PermissiveGuard, PedanticGuard, Guard};
use self::super::base::{transmute_many, from_bytes};
#[cfg(feature = "std")]
use self::super::base::transmute_vec;
use self::super::Error;


/// Type that can be constructed from any combination of bytes.
///
/// A type `T` implementing this trait means that any arbitrary slice of bytes
/// of length `size_of::<T>()` can be safely interpreted as a value of that
/// type with support for unaligned memory access. In most (but not all)
/// cases this is a [*POD class*](http://eel.is/c++draft/class#10) or a
/// [*trivially copyable class*](http://eel.is/c++draft/class#6).
///
/// This serves as a marker trait for all functions in this module.
///
/// *Warning*: if you transmute into a floating-point type you will have a chance to create a signaling NaN,
/// which, while not illegal, can be unwieldy. Check out [`util::designalise_f{32,64}()`](util/index.html)
/// for a remedy.
///
/// *Nota bene*: `bool` is not `TriviallyTransmutable` because they're restricted to
/// being `0` or `1`, which means that an additional value check is required.
///
/// # Safety
///
/// It is only safe to implement `TriviallyTransmutable` for a type `T` if it
/// is safe to read or write a value `T` at the pointer of an arbitrary slice
/// `&[u8]`, of length `size_of<T>()`, as long as the same slice is
/// *well aligned* in memory for reading and writing a `T`.
///
/// Consult the [Transmutes section](https://doc.rust-lang.org/nomicon/transmutes.html)
/// of the Nomicon for more details.
pub unsafe trait TriviallyTransmutable: Copy {}


unsafe impl TriviallyTransmutable for u8 {}
unsafe impl TriviallyTransmutable for i8 {}
unsafe impl TriviallyTransmutable for u16 {}
unsafe impl TriviallyTransmutable for i16 {}
unsafe impl TriviallyTransmutable for u32 {}
unsafe impl TriviallyTransmutable for i32 {}
unsafe impl TriviallyTransmutable for u64 {}
unsafe impl TriviallyTransmutable for i64 {}
unsafe impl TriviallyTransmutable for usize {}
unsafe impl TriviallyTransmutable for isize {}
unsafe impl TriviallyTransmutable for f32 {}
unsafe impl TriviallyTransmutable for f64 {}
#[cfg(i128_type)]
unsafe impl TriviallyTransmutable for u128 {}
#[cfg(i128_type)]
unsafe impl TriviallyTransmutable for i128 {}

unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 1] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 2] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 3] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 4] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 5] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 6] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 7] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 8] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 9] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 10] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 11] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 12] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 13] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 14] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 15] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 16] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 17] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 18] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 19] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 20] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 21] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 22] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 23] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 24] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 25] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 26] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 27] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 28] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 29] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 30] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 31] {}
unsafe impl<T: TriviallyTransmutable> TriviallyTransmutable for [T; 32] {}


/// Transmute a byte slice into a single instance of a trivially transmutable type.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have enough bytes for a single value `T`.
///
/// # Safety
///
/// This function invokes undefined behavior if the data does not have a memory
/// alignment compatible with `T`. If this cannot be ensured, you will have to
/// make a copy of the data, or change how it was originally made.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::trivial::transmute_trivial;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(transmute_trivial::<u32>(&[0x00, 0x00, 0x00, 0x01])?, 0x0100_0000);
/// # */
/// #   assert_eq!(transmute_trivial::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x0100_0000);
/// }
/// # }
/// ```
pub unsafe fn transmute_trivial<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<T, Error<u8, T>> {
    from_bytes::<T>(bytes)
}

/// Transmute a byte slice into a single instance of a trivially transmutable type.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Errors
///
/// An error is returned in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not have enough bytes for a single value `T`.
/// - The data has more bytes than those required to produce a single value `T`.
///
/// # Safety
///
/// This function invokes undefined behavior if the data does not have a memory
/// alignment compatible with `T`. If this cannot be ensured, you will have to
/// make a copy of the data, or change how it was originally made.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::trivial::transmute_trivial_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(transmute_trivial_pedantic::<u16>(&[0x0F, 0x0E])?, 0x0E0F);
/// # */
/// #   assert_eq!(transmute_trivial_pedantic::<u16>(&[0x0F, 0x0E].le_to_native::<u16>()).unwrap(), 0x0E0F);
/// }
/// # }
/// ```
pub unsafe fn transmute_trivial_pedantic<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<T, Error<u8, T>> {
    PedanticGuard::check::<T>(bytes)?;
    from_bytes(bytes)
}

/// Transmute a byte slice into a single instance of a trivially transmutable type.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Errors
///
/// An error is returned if the data does not comply with the policies of the
/// given guard `G`.
///
/// # Safety
///
/// This function invokes undefined behavior if the data does not have a memory
/// alignment compatible with `T`. If this cannot be ensured, you will have to
/// make a copy of the data, or change how it was originally made.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::trivial::transmute_trivial_many;
/// # use safe_transmute::SingleManyGuard;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(transmute_trivial_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02])?,
/// # */
/// #   assert_eq!(transmute_trivial_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///                &[0x0100, 0x0200]);
/// }
/// # }
/// ```
pub unsafe fn transmute_trivial_many<T: TriviallyTransmutable, G: Guard>(bytes: &[u8]) -> Result<&[T], Error<u8, T>> {
    transmute_many::<T, G>(bytes)
}

/// View a byte slice as a slice of a trivially transmutable type.
///
/// The resulting slice will have as many instances of a type as will fit, rounded down.
#[deprecated(since = "0.11.0", note = "see `trivial::transmute_many()` with `PermissiveGuard` for the equivalent behavior")]
pub unsafe fn guarded_transmute_pod_many_permissive<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<&[T], Error<u8, T>> {
    Ok(transmute_many::<T, PermissiveGuard>(bytes)?)
}

/// View a byte slice as a slice of a trivially transmutable type.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// and should not have extraneous data.
#[deprecated(since = "0.11.0", note = "see `trivial::transmute_many()` with `PedanticGuard` for the equivalent behavior")]
pub unsafe fn guarded_transmute_pod_many_pedantic<T: TriviallyTransmutable>(bytes: &[u8]) -> Result<&[T], Error<u8, T>> {
    transmute_many::<T, PedanticGuard>(bytes)
}


/// Transform a vector into a vector of another element type.
///
/// The vector's allocated byte buffer (if already allocated) will be reused.
///
/// # Safety
///
/// Vector transmutations are **exceptionally** dangerous because of
/// the constraints imposed by
/// [`Vec::from_raw_parts`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts).
///
/// Unless *all* of the following requirements are fulfilled, this operation
/// may result in undefined behavior.
///
/// - The target type `T` must have the same size and minimum memory alignment
///   requirements as the type `S`.
///
/// # Examples
///
/// ```
/// # use safe_transmute::trivial::transmute_trivial_vec;
/// unsafe {
///     assert_eq!(
///         transmute_trivial_vec::<u8, i8>(vec![0x00, 0x01, 0x00, 0x02]),
///         vec![0x00, 0x01, 0x00, 0x02]
///     );
/// }
/// ```
#[cfg(feature = "std")]
pub unsafe fn transmute_trivial_vec<S: TriviallyTransmutable, T: TriviallyTransmutable>(vec: Vec<S>) -> Vec<T> {
    transmute_vec::<S, T>(vec)
}

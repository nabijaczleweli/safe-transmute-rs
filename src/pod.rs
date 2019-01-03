//! Plain data object safe transmute
//!
//! Functions in this module are guarded from out-of-bounds memory access and
//! from unsafe transmutation target types through the use of the
//! [`PodTransmutable`](trait.PodTransmutable.html)) trait.
//! 
//! However, they are still not entirely safe because the source data may not
//! be correctly aligned for reading and writing a value of the target type.
//! The effects of this range from less performance (e.g. x86) to trapping or
//! address flooring (e.g. ARM), but this is undefined behavior nonetheless.


use crate::Error;
#[cfg(feature = "std")]
use crate::base::guarded_transmute_vec;
use crate::base::{guarded_transmute_many, from_bytes};
use crate::guard::{Guard, PedanticGuard, PermissiveGuard};


/// Type that can be non-`unsafe`ly transmuted into
///
/// A type `T` implementing this trait means that any arbitrary slice of bytes
/// of length `size_of::<T>()` can be safely interpreted as a value of that
/// type in all circumstances. In most cases this is a
/// [*POD class*](http://eel.is/c++draft/class#10) or a
/// [*trivially copyable class*](http://eel.is/c++draft/class#6).
///
/// This serves as a marker trait for all functions in this module.
///
/// *Warning*: if you transmute into a floating-point type you will have a chance to create a signaling NaN,
/// which, while not illegal, can be unwieldy. Check out [`util::designalise_f{32,64}()`](util/index.html)
/// for a remedy.
///
/// *Nota bene*: `bool` is not `PodTransmutable` because they're restricted to
/// being `0` or `1`, which means that an additional value check is required.
///
/// # Safety
///
/// It is only safe to implement `PodTransmutable` for a type `T` if it is safe for a slice of any arbitrary data
/// `&[u8]` of length `sizeof<T>()` to be [`transmute()`](https://doc.rust-lang.org/stable/std/mem/fn.transmute.html)d
/// to a unit-length `&[T]`, without any other conversion operation being required.
///
/// Consult the [Transmutes section](https://doc.rust-lang.org/nomicon/transmutes.html) of the Nomicon for more details.
pub unsafe trait PodTransmutable: Copy {}


unsafe impl PodTransmutable for u8 {}
unsafe impl PodTransmutable for i8 {}
unsafe impl PodTransmutable for u16 {}
unsafe impl PodTransmutable for i16 {}
unsafe impl PodTransmutable for u32 {}
unsafe impl PodTransmutable for i32 {}
unsafe impl PodTransmutable for u64 {}
unsafe impl PodTransmutable for i64 {}
unsafe impl PodTransmutable for usize {}
unsafe impl PodTransmutable for isize {}
unsafe impl PodTransmutable for f32 {}
unsafe impl PodTransmutable for f64 {}
#[cfg(i128_type)]
unsafe impl PodTransmutable for u128 {}
#[cfg(i128_type)]
unsafe impl PodTransmutable for i128 {}

unsafe impl<T: PodTransmutable> PodTransmutable for [T; 1] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 2] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 3] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 4] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 5] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 6] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 7] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 8] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 9] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 10] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 11] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 12] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 13] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 14] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 15] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 16] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 17] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 18] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 19] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 20] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 21] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 22] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 23] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 24] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 25] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 26] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 27] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 28] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 29] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 30] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 31] {}
unsafe impl<T: PodTransmutable> PodTransmutable for [T; 32] {}


/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Errors
///
/// An error is raised in one of the following situations:
///
/// - The data does not have enough bytes for a single value `T`.
///
/// # Safety
/// 
/// It is undefined behavior If the data does not have a memory alignment
/// compatible with `T`. If this cannot be ensured, you will have to make a
/// copy of the data, or change how it was originally made.
/// 
/// # Examples
///
/// ```no_run
/// # use safe_transmute::pod::guarded_transmute_pod;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
/// assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01])?, 0x0100_0000);
/// # */
/// # assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x0100_0000);
/// }
/// # }
/// ```
pub unsafe fn guarded_transmute_pod<T: PodTransmutable>(bytes: &[u8]) -> Result<T, Error> {
    from_bytes::<T>(bytes)
}

/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Errors
///
/// An error is raised in one of the following situations:
///
/// - The data does not have a memory alignment compatible with `T`. You will
///   have to make a copy anyway, or modify how the data was originally made.
/// - The data does not have enough bytes for a single value `T`.
/// - The data has more bytes than those required to produce a single value `T`.
///
/// # Safety
/// 
/// It is undefined behavior If the data does not have a memory alignment
/// compatible with `T`. If this cannot be ensured, you will have to make a
/// copy of the data, or change how it was originally made.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::pod::guarded_transmute_pod_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(guarded_transmute_pod_pedantic::<u16>(&[0x0F, 0x0E])?, 0x0E0F);
/// # */
/// #   assert_eq!(guarded_transmute_pod_pedantic::<u16>(&[0x0F, 0x0E].le_to_native::<u16>()).unwrap(), 0x0E0F);
/// }
/// # }
/// ```
pub unsafe fn guarded_transmute_pod_pedantic<T: PodTransmutable>(bytes: &[u8]) -> Result<T, Error> {
    PedanticGuard::check::<T>(bytes)?;
    from_bytes(bytes)
}

/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Errors
///
/// An error is raised in one of the following situations:
///
/// - The data does not have enough bytes for a single value `T`.
///
/// # Safety
/// 
/// It is undefined behavior If the data does not have a memory alignment
/// compatible with `T`. If this cannot be ensured, you will have to make a
/// copy of the data, or change how it was originally made.
///
/// # Examples
///
/// ```no_run
/// use safe_transmute::pod::guarded_transmute_pod_many;
/// use safe_transmute::SingleManyGuard;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(guarded_transmute_pod_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02])?,
/// # */
/// #   assert_eq!(guarded_transmute_pod_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///                &[0x0100, 0x0200]);
/// }
/// # }
/// ```
pub unsafe fn guarded_transmute_pod_many<T, G>(bytes: &[u8]) -> Result<&[T], Error>
where
    T: PodTransmutable,
    G: Guard,
{
    guarded_transmute_many::<T, G>(bytes)
}

/// View a byte slice as a slice of a POD type.
///
/// The resulting slice will have as many instances of a type as will fit, rounded down.
#[deprecated(since = "0.11.0", note = "see `pod::guarded_transmute_pod_many` for the equivalent behavior")]
pub unsafe fn guarded_transmute_pod_many_permissive<T: PodTransmutable>(bytes: &[u8]) -> Result<&[T], Error> {
    Ok(guarded_transmute_many::<T, PermissiveGuard>(bytes)?)
}

/// View a byte slice as a slice of POD.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// and should not have extraneous data.
///
/// # Errors
///
/// An error is raised in one of the following situations:
///
/// - The data does not have enough bytes for a single value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::pod::guarded_transmute_pod_many_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B])?,
/// # */
/// #   assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B].le_to_native::<u16>()).unwrap(),
///                &[0x0E0F, 0x0B0A]);
/// }
/// # }
/// ```
#[deprecated(since = "0.11.0", note = "see `pod::guarded_transmute_pod_many` for the equivalent behavior")]
pub unsafe fn guarded_transmute_pod_many_pedantic<T: PodTransmutable>(bytes: &[u8]) -> Result<&[T], Error> {
    guarded_transmute_many::<T, PedanticGuard>(bytes)
}

/// Transform a byte vector into a vector of POD.
///
/// The resulting vec will reuse the allocated byte buffer when possible, and
/// should have at least enough bytes to fill a single instance of a type.
/// Extraneous data is ignored.
///
/// # Errors
///
/// An error is raised in one of the following situations:
///
/// - The data does not have enough bytes for a single value `T`.
///
/// # Safety
/// 
/// It is undefined behavior If the data does not have a memory alignment
/// compatible with `T`. If this cannot be ensured, you will have to make a
/// copy of the data, or change how it was originally made.
///
/// # Examples
///
/// ```no_run
/// use safe_transmute::pod::guarded_transmute_pod_vec;
/// use safe_transmute::SingleManyGuard;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(guarded_transmute_pod_vec::<u16, SingleManyGuard>(vec![0x00, 0x01, 0x00, 0x02])?,
/// # */
/// #   assert_eq!(guarded_transmute_pod_vec::<u16, SingleManyGuard>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
/// # /*
///     assert_eq!(guarded_transmute_pod_vec::<u32, SingleManyGuard>(vec![0x04, 0x00, 0x00, 0x00, 0xED])?,
/// # */
/// #   assert_eq!(guarded_transmute_pod_vec::<u32, SingleManyGuard>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()).unwrap(),
///            vec![0x0000_0004]);
///
///     assert!(guarded_transmute_pod_vec::<i16, SingleManyGuard>(vec![0xED]).is_err());
/// }
/// # }
/// ```
#[cfg(feature = "std")]
pub unsafe fn guarded_transmute_pod_vec<T, G>(bytes: Vec<u8>) -> Result<Vec<T>, Error>
where
    T: PodTransmutable,
    G: Guard,
{
    guarded_transmute_vec::<T, G>(bytes)
}

/// Transform a byte vector into a vector of POD.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// have as many instances of a type as will fit, rounded down.
/// Extraneous data is ignored.
///
/// # Safety
/// 
/// It is undefined behavior If the data does not have a memory alignment
/// compatible with `T`. If this cannot be ensured, you will have to make a
/// copy of the data, or change how it was originally made.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::pod::guarded_transmute_pod_vec_permissive;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02]),
/// # */
/// #     assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
///                Ok(vec![0x0100, 0x0200]));
/// # /*
///     assert_eq!(guarded_transmute_pod_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]),
/// # */
/// #     assert_eq!(guarded_transmute_pod_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()),
///                Ok(vec![0x0000_0004]));
///     assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![0xED]), Ok(vec![]));
/// }
/// # }
/// ```
#[cfg(feature = "std")]
#[deprecated(since = "0.11.0", note = "see `pod::guarded_transmute_pod_vec` for the equivalent behavior")]
pub unsafe fn guarded_transmute_pod_vec_permissive<T: PodTransmutable>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    guarded_transmute_vec::<T, PermissiveGuard>(bytes).map_err(From::from)
}

/// Transform a byte vector into a vector of POD.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not have extraneous data.
///
/// # Safety
/// 
/// It is undefined behavior If the data does not have a memory alignment
/// compatible with `T`. If this cannot be ensured, you will have to make a
/// copy of the data, or change how it was originally made.
///
/// # Errors
///
/// An error is raised in one of the following situations:
///
/// - The data does not have enough bytes for a single value `T`.
/// - The last chunk of the size of `T` is not large enough for a value, leaving extraneous bytes.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::pod::guarded_transmute_pod_vec_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02])?,
/// # */
/// #     assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///                  vec![0x0100, 0x0200]);
///
///     assert!(guarded_transmute_pod_vec_pedantic::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED])
///               .is_err());
/// }
/// # }
/// ```
#[cfg(feature = "std")]
#[deprecated(since = "0.11.0", note = "see `pod::guarded_transmute_pod_vec` for the equivalent behavior")]
pub unsafe fn guarded_transmute_pod_vec_pedantic<T: PodTransmutable>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    guarded_transmute_vec::<T, PedanticGuard>(bytes)
}

use self::super::{Error, guarded_transmute_vec_permissive, guarded_transmute_many_permissive, guarded_transmute_many_pedantic, guarded_transmute_vec_pedantic,
                  guarded_transmute_pedantic, guarded_transmute_vec, guarded_transmute_many, guarded_transmute};


/// Type that can be non-`unsafe`ly transmuted into
///
/// In most cases this is a [*POD class*](http://eel.is/c++draft/class#10)
/// or a [*trivially copyable class*](http://eel.is/c++draft/class#6).
///
/// Marker trait for `guarded_transmute_pod_*()` functions.
///
/// *Warning*: if you transmute into a floating-point type you will have a chance to create a signaling NaN,
/// which, while not illegal, can be unwieldy. Check out [`util::designalise_f{32,64}()`](util/) for a remedy.
///
/// *Nota bene*: `bool`s aren't *actually* non-`unsafe` to transmute, because they're restricted to being `0` or `1`,
/// which means it's UB to transmute an arbitrary byte into a `bool`.
pub trait PodTransmutable {}

impl PodTransmutable for u8 {}
impl PodTransmutable for i8 {}
impl PodTransmutable for u16 {}
impl PodTransmutable for i16 {}
impl PodTransmutable for u32 {}
impl PodTransmutable for i32 {}
impl PodTransmutable for u64 {}
impl PodTransmutable for i64 {}
impl PodTransmutable for char {}
impl PodTransmutable for f32 {}
impl PodTransmutable for f64 {}
#[cfg(i128_type)]
impl PodTransmutable for u128 {}
#[cfg(i128_type)]
impl PodTransmutable for i128 {}


/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01]).unwrap(), 0x01000000);
/// # */
/// # assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x01000000);
/// # }
/// ```
pub fn guarded_transmute_pod<T: PodTransmutable + Copy>(bytes: &[u8]) -> Result<T, Error> {
    unsafe { guarded_transmute(bytes) }
}

/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(guarded_transmute_pod_pedantic::<u16>(&[0x0F, 0x0E]).unwrap(), 0x0E0F);
/// # */
/// # assert_eq!(guarded_transmute_pod_pedantic::<u16>(&[0x0F, 0x0E].le_to_native::<u16>()).unwrap(), 0x0E0F);
/// # }
/// ```
pub fn guarded_transmute_pod_pedantic<T: PodTransmutable + Copy>(bytes: &[u8]) -> Result<T, Error> {
    unsafe { guarded_transmute_pedantic(bytes) }
}

/// Transmute a byte slice into a single instance of a POD.
///
/// The byte slice must have exactly enough bytes to fill a single instance of a type.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod_many;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(guarded_transmute_pod_many::<u16>(&[0x00, 0x01, 0x00, 0x02]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_pod_many::<u16>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            &[0x0100, 0x0200]);
/// # }
/// ```
pub fn guarded_transmute_pod_many<T: PodTransmutable>(bytes: &[u8]) -> Result<&[T], Error> {
    unsafe { guarded_transmute_many(bytes) }
}

/// View a byte slice as a slice of a POD type.
///
/// The resulting slice will have as many instances of a type as will fit, rounded down.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod_many_permissive;
/// assert_eq!(guarded_transmute_pod_many_permissive::<u16>(&[0x00]), &[]);
/// ```
pub fn guarded_transmute_pod_many_permissive<T: PodTransmutable>(bytes: &[u8]) -> &[T] {
    unsafe { guarded_transmute_many_permissive(bytes) }
}

/// View a byte slice as a slice of POD.
///
/// The byte slice must have at least enough bytes to fill a single instance of a type,
/// and should not have extraneous data.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod_many_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x0F, 0x0E, 0x0A, 0x0B].le_to_native::<u16>()).unwrap(),
///            &[0x0E0F, 0x0B0A]);
/// # }
/// ```
pub fn guarded_transmute_pod_many_pedantic<T: PodTransmutable>(bytes: &[u8]) -> Result<&[T], Error> {
    unsafe { guarded_transmute_many_pedantic(bytes) }
}

/// Trasform a byte vector into a vector of POD.
///
/// The resulting vec will reuse the allocated byte buffer when possible, and
/// should have at least enough bytes to fill a single instance of a type.
/// Extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod_vec;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(guarded_transmute_pod_vec::<u16>(vec![0x00, 0x01, 0x00, 0x02]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_pod_vec::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
/// # /*
/// assert_eq!(guarded_transmute_pod_vec::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_pod_vec::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()).unwrap(),
///            vec![0x00000004]);
///
/// assert!(guarded_transmute_pod_vec::<i16>(vec![0xED]).is_err());
/// # }
/// ```
pub fn guarded_transmute_pod_vec<T: PodTransmutable>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    unsafe { guarded_transmute_vec(bytes) }
}

/// Trasform a byte vector into a vector of POD.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// have as many instances of a type as will fit, rounded down.
/// Extraneous data is ignored.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod_vec_permissive;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02]),
/// # */
/// # assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
///            vec![0x0100, 0x0200]);
/// # /*
/// assert_eq!(guarded_transmute_pod_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]),
/// # */
/// # assert_eq!(guarded_transmute_pod_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()),
///            vec![0x00000004]);
/// assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![0xED]), vec![]);
/// # }
/// ```
pub fn guarded_transmute_pod_vec_permissive<T: PodTransmutable>(bytes: Vec<u8>) -> Vec<T> {
    unsafe { guarded_transmute_vec_permissive(bytes) }
}


/// Trasform a byte vector into a vector of POD.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not have extraneous data.
///
/// # Examples
///
/// ```
/// # use safe_transmute::guarded_transmute_pod_vec_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// # /*
/// assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02]).unwrap(),
/// # */
/// # assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///            vec![0x0100, 0x0200]);
///
/// assert!(guarded_transmute_pod_vec_pedantic::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED])
///           .is_err());
/// # }
/// ```
pub fn guarded_transmute_pod_vec_pedantic<T: PodTransmutable>(bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    unsafe { guarded_transmute_vec_pedantic(bytes) }
}

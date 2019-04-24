//! This crate contains checked implementations of transmutation procedures, some of which
//! ensure memory safety.
//!
//! ## Crate outline
//!
//! The following modules are available:
//!
//! - The functions in the [`base`](base/index.html) module are not inherently
//!   safe, but just protected against out of boundary access (like trying to
//!   create an 8-byte type from 7 bytes). These functions are as safe as
//!   the data passed to them: any attempt of transmuting data to an invalid
//!   memory representation is still undefined behavior. Moreover, unaligned
//!   memory access is not prevented, and must be previously ensured by the
//!   caller.
//! - The [`guard`](guard/index.html) module contains the **Guard API**, which
//!   imposes slice boundary restrictions in a conversion.
//! - The [`trivial`](trivial/index.html) module introduces the
//!   [`TriviallyTransmutable`](trivial/trait.TriviallyTransmutable.html)
//!   trait, which statically ensures that any bit combination makes a valid
//!   value for a given type. The functions in this module are safer than
//!   [`base`](base/index.html), but still do not prevent unaligned memory access.
//! - [`to_bytes`](to_bytes/index.html) enables the opposite operation of
//!   reintepreting values as bytes.
//! - The [`bool`](bool/index.html) module ensures safe transmutation of bytes
//!   to boolean values.
//! - At the root of this crate, there are transmutation functions with enough
//!   checks to be considered safe to use in any circumstance. The operation may
//!   still arbitrarily return (recoverable) errors due to unaligned data or
//!   incompatible vector transmutation targets, but it will not eat your
//!   laundry, and helper functions are available to assist the programmer in
//!   making some use cases work.
//!
//! This crate can be used in a no-`std` environment by disabling the `std`
//! feature through specifying `default-features = false` on import.
//!
//! # Examples
//!
//! View bytes as a series of `u16`s, with a single-many boundary
//! guard (at least one value, extraneous bytes are allowed):
//!
//! ```
//! # use safe_transmute::{transmute_many, SingleManyGuard, Error};
//! # include!("../tests/test_util/le_to_native.rs");
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<::std::error::Error>> {
//! let bytes = &[0x00, 0x01, 0x12, 0x24,
//!     0x00]; // spare byte, unused
//! match transmute_many::<u16, SingleManyGuard>(bytes) {
//!     Ok(hwords) => {
//!         assert_eq!(
//!             hwords,
//!             &[
//!                 u16::from_be(0x0001),
//!                 u16::from_be(0x1224),
//!             ][..]);
//!     },
//!     Err(Error::Unaligned(e)) => {
//!         // whelp, we need a copy here
//!         let hwords = e.copy();
//!         assert_eq!(
//!             &*hwords,
//!             &[
//!                 u16::from_be(0x0001),
//!                 u16::from_be(0x1224),
//!             ][..]);
//!     },
//!     Err(e) => panic!("We are not expecting this: {}", e),
//! }
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! ```
//!
//! Since one may not always be able to ensure that a slice of bytes is well
//! aligned for reading data of different constraints, such as from `u8` to
//! `u16`, the operation may fail without a trivial means of prevention.
//! As a remedy, the data can be copied into a new vector with the help of the
//! [`try_copy!`](macro.try_copy.html) macro.
//! 
//! ```
//! # #[macro_use]
//! # extern crate safe_transmute;
//! # use safe_transmute::{transmute_many, SingleManyGuard, Error};
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<::std::error::Error>> {
//! # let bytes = &[0x00, 0x01, 0x12, 0x24, 0x00];
//! let hwords = try_copy!(transmute_many::<u16, SingleManyGuard>(bytes));
//! assert_eq!(
//!     &*hwords,
//!     &[
//!         u16::from_be(0x0001),
//!         u16::from_be(0x1224),
//!     ][..]);
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! ```
//! 
//! View all bytes as a series of `u16`s:
//!
//! ```
//! # #[macro_use]
//! # extern crate safe_transmute;
//! # use safe_transmute::{transmute_many_pedantic, Error};
//! # include!("../tests/test_util/le_to_native.rs");
//! # #[cfg(feature = "std")]
//! # fn main() -> Result<(), Box<::std::error::Error>> {
//! // assuming Little Endian machine
//! # let bytes = [0x00, 0x01, 0x12, 0x34].le_to_native::<u16>();
//! # let bytes = &bytes;
//! # let transmuted = try_copy!(transmute_many_pedantic::<u16>(bytes).map_err(|e| e.without_src()));
//! # /*
//! let bytes = &[0x00, 0x01, 0x12, 0x34];
//! let transmuted = try_copy!(transmute_many_pedantic::<u16>(bytes));
//! # */
//! 
//! assert_eq!(&*transmuted, &[0x0100, 0x3412][..]);
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! ```
//!
//! View a byte slice as a single `f64`:
//!
//! ```no_run
//! # use safe_transmute::transmute_one;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! assert_eq!(transmute_one::<f64>(
//!     &[0x00, 0x00, 0x00, 0x00,
//! # /*
//!       0x00, 0x00, 0x00, 0x40])?,
//! # */
//! #     0x00, 0x00, 0x00, 0x40].le_to_native::<f64>()).unwrap(),
//!     2.0);
//! # }
//! ```
//!
//! View a series of `u16`s as bytes:
//!
//! ```
//! # use safe_transmute::transmute_to_bytes;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! assert_eq!(transmute_to_bytes(&[0x0001u16,
//!                                      0x1234u16]),
//! # /*
//!            &[0x01, 0x00, 0x34, 0x12].le_to_native::<u16>());
//! # */
//! #          &[0x01, 0x00, 0x34, 0x12].le_to_native::<u16>());
//! # }
//! ```


#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(feature = "std")]
extern crate core;

mod full;

pub mod align;
pub mod base;
pub mod bool;
pub mod error;
pub mod guard;
pub mod trivial;
pub mod util;
pub mod to_bytes;

pub use self::full::{transmute_many_permissive, transmute_many_pedantic, transmute_one_pedantic, transmute_many, transmute_one};
#[cfg(feature = "std")]
pub use self::full::transmute_vec;


pub use self::guard::{SingleValueGuard, PermissiveGuard, SingleManyGuard, PedanticGuard, Guard};
pub use self::error::{UnalignedError, ErrorReason, GuardError, Error};
#[cfg(feature = "std")]
pub use self::error::IncompatibleVecTargetError;
pub use self::trivial::TriviallyTransmutable;

pub use self::to_bytes::{transmute_one_to_bytes, transmute_to_bytes};
#[cfg(feature = "std")]
pub use self::to_bytes::transmute_to_bytes_vec;

#[cfg(feature = "std")]
pub use self::bool::{transmute_bool_vec_permissive, transmute_bool_vec_pedantic};
pub use self::bool::{transmute_bool_permissive, transmute_bool_pedantic};

/// Retrieve the result of a transmutation, copying the data if this cannot be
/// done safely due to memory alignment constraints.
/// 
/// The macro, not unlike `try!`, will short-circuit certain errors with
/// `return`, namely guard condition and invalid value errors. When the operation
/// fails due to either an unaligned transmutation or an incompatible vector
/// element transmutation target, the transmutation is once again attempted by
/// copying the output into a vector.
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
    ($res: expr) => {
            $res.map_err(::safe_transmute::Error::from)
                .map(::std::borrow::Cow::from)
                .or_else(|e| e.copy().map(::std::borrow::Cow::Owned))?
    };
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
    ($res: expr) => {
        $res.map_err(::safe_transmute::Error::from)
            .map(::std::borrow::Cow::from)
            .or_else(|e| e.copy_unchecked().map(::std::borrow::Cow::Owned))?
    };
}

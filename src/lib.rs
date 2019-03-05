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
//!   to boolean values and vice versa.
//! - At the root of this crate, there are transmutation functions with enough
//!   checks to be considered safe to use in any circumstance. The operation may
//!   still arbitrarily return (recoverable) errors due to unaligned data, but it
//!   will not eat your laundry.
//!
//! This crate can be used in a no-`std` environment by disabling the `std`
//! feature through specifying `default-features = false` on import.
//!
//! # Examples
//!
//! View bytes as a series of `u16`s, with a single-many boundary
//! guard (at least one value, extraneous bytes are allowed):
//!
//! ```no_run
//! # use safe_transmute::{transmute_many, SingleManyGuard};
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! assert_eq!(transmute_many::<u16, SingleManyGuard>(
//!     &[0x00, 0x01, 0x12, 0x34,
//!       // Spare byte, unused
//! # /*
//!       0x00])?,
//! # */
//! #     0x00].le_to_native::<u16>()).unwrap(),
//!     &[0x0100, 0x3412]);
//! # }
//! ```
//!
//! View all bytes as a series of `u16`s:
//!
//! ```no_run
//! # use safe_transmute::transmute_many_pedantic;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! assert_eq!(transmute_many_pedantic::<u16>(
//!     &[0x00, 0x01,
//! # /*
//!       0x12, 0x34])?,
//! # */
//! #     0x12, 0x34].le_to_native::<u16>()).unwrap(),
//!     &[0x0100, 0x3412]);
//! # }
//! ```
//!
//! View a byte slice as a single `f64`:
//!
//! ```no_run
//! # use safe_transmute::transmute_one;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! # unsafe {
//! assert_eq!(transmute_one::<f64>(
//!     &[0x00, 0x00, 0x00, 0x00,
//! # /*
//!       0x00, 0x00, 0x00, 0x40])?,
//! # */
//! #     0x00, 0x00, 0x00, 0x40].le_to_native::<f64>()).unwrap(),
//!     2.0);
//! # }
//! # }
//! ```
//!
//! View a series of `u16`s as bytes:
//!
//! ```
//! # use safe_transmute::transmute_to_bytes;
//! # include!("../tests/test_util/le_to_native.rs");
//! # fn main() {
//! # unsafe {
//! assert_eq!(transmute_to_bytes(&[0x0001u16,
//!                                      0x1234u16]),
//! # /*
//!            &[0x01, 0x00, 0x34, 0x12].le_to_native::<u16>());
//! # */
//! #          &[0x01, 0x00, 0x34, 0x12].le_to_native::<u16>());
//! # }
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
pub use self::full::{transmute_vec_permissive, transmute_vec_pedantic, transmute_vec};


pub use self::guard::{SingleValueGuard, PermissiveGuard, SingleManyGuard, PedanticGuard, Guard};
pub use self::error::{ErrorReason, GuardError, Error};
pub use self::trivial::TriviallyTransmutable;

pub use self::to_bytes::{transmute_one_to_bytes, transmute_to_bytes};
#[cfg(feature = "std")]
pub use self::to_bytes::transmute_to_bytes_vec;

#[cfg(feature = "std")]
pub use self::bool::{transmute_bool_vec_permissive, transmute_bool_vec_pedantic};
pub use self::bool::{transmute_bool_permissive, transmute_bool_pedantic};

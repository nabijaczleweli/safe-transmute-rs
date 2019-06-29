//! Migrating to `safe-transmute` v0.11
//!
//! This guide starts with a forewarning: `safe-transmute` had many safety issues before this version,
//! which means that there is a chance of your dependent project facing undefined behavior. Migrating
//! to version 0.11 is recommended as soon as possible, even if it might lead to a sub-optimal solution.
//!
//! ## Organization
//!
//! The crate is now organized with the following major categories:
//!   - `base` contains all baseline conversion functions. They are only protected against out of boundary
//!     access, like trying to create an 8-byte type from 7 bytes. However, they are still unsafe: any
//!     attempt of transmuting data to an invalid memory representation is still undefined behavior.
//!     Moreover, unaligned memory access is not prevented, and must be previously ensured by the caller.
//!   - The `trivial` module introduces the concept of being *trivially transmutable*, which
//!     statically ensures that any bit combination makes a valid value for a given type. Basically,
//!     if a type `T` can be filled with any arbitrary bits in its memory representation and still be
//!     valid, then `T` is trivially transmutable. Most primitive types implement the `TriviallyTransmutable`
//!     trait, as well as arrays of trivially transmutable types, but new types (such as repr-C structs)
//!     need to `unsafe impl` it manually. Functions in this module are therefore safer than the baseline,
//!     but are still unsafe because they do not check for memory alignment.
//!   - `to_bytes` enables the opposite operation of reintepreting values as bytes. They are usually safe
//!     unless when working with mutable slices, since they can break invariants of the source type.
//!   - The `bool` module ensures safe transmutation of bytes to boolean values.
//!   - That leaves the `full` functions at the crate root. These are transmutation functions with enough
//!     checks to be considered safe to use in any circumstance. The operation may still arbitrarily
//!     return (recoverable) errors due to unaligned data or incompatible vector transmutation targets,
//!     but it will not eat your laundry, and helper functions are available to assist the user in
//!     making some use cases work.
//!
//! Moreover, three utility modules have also been provided:
//!   - The `guard` module contains the **Guard API**, which imposes slice boundary restrictions in a conversion.
//!   - The `align` module is where alignment checks are implemented.
//!   - The `util` module provides some independent helper functions.
//!
//! Generally, you are strongly advised to *stick to the functions provided at the crate root*.
//! These are re-exports from the `full`, `bool`, and `to_bytes` categories depending on their safety.
//!
//! ## Transmuting slices
//!
//! One of the major use cases of the crate is to grab a slice of bytes and reinterpret it as a slice of
//! another type. This process is accompanied with a check for the source slice length, so that it
//! makes some sense as the target type. If you expect any number of elements of the target type, use
//! `transmute_many_permissive()`.
//!
//! ```rust
//! # #[cfg(feature = "alloc")]
//! # {
//! use safe_transmute::{Error, transmute_many_permissive};
//!
//! let bytes = &[0x00, 0x01, 0x12, 0x24,
//!               0x00]; // 1 spare byte
//!
//! match transmute_many_permissive::<u16>(bytes) {
//!     Ok(words) => {
//!         assert_eq!(words,
//!                    [u16::from_be(0x0001), u16::from_be(0x1224)]);
//!     },
//!     Err(Error::Unaligned(e)) => {
//!         // Copy needed, would otherwise trap on some archs
//!         let words = e.copy();
//!         assert_eq!(*words,
//!                    [u16::from_be(0x0001), u16::from_be(0x1224)]);
//!     },
//!     Err(e) => panic!("Unexpected error: {}", e),
//! }
//! # }
//! ```
//!
//! `transmute_many_permissive()` is an alias for `transmute_many()` with `PermissiveGuard`
//! as the guard type parameter. If you expect at least 1 element, use `transmute_many()` with the
//! `SingleManyGuard` as the guard type. If you expect at least one element and no extraneous bytes, use
//! `transmute_many_pedantic()`, or `transmute_many()` with `PedanticGuard`.
//!
//! As you can see, we had to manually handle the case where the slice of bytes is not well aligned for
//! reading target data, such as from `u8` to `u16`. If the slice's first element is not aligned for
//! reading `u16`s, the operation will just fail with `Error::Unaligned`. The only way to move on from
//! here is to copy the data (provided by the `Error::copy()` method).
//!
//! The good news is that this boilerplate can be off-loaded to the `try_copy!` macro. Here's how you'll
//! often be doing transmutations:
//!
//! ```rust
//! # use safe_transmute::Error;
//! # #[cfg(feature = "alloc")]
//! # {
//! use safe_transmute::{transmute_many_permissive, try_copy};
//!
//! let bytes = &[0x00, 0x01, 0x12, 0x24, 0x00];
//! let words = try_copy!(transmute_many_permissive::<u16>(bytes));
//!
//! assert_eq!(&*words, &[u16::from_be(0x0001), u16::from_be(0x1224)]);
//! # }
//! # Ok::<(), Error<u8, u16>>(())
//! ```
//!
//! You will also find `to_bytes`, `mut`, and `bool` variants of these functions. `transmute_to_bytes()` turns any slice into
//! a slice of bytes. Functions from the `*_mut()` family work with mutable slices. `transmute_bool()` checks whether all bytes
//! make valid boolean values beforehand.
//!
//! ## Transmuting vectors
//!
//! You might have used safe-transmute's vector transmutation functions. Well, it turns out that they are
//! **incredibly unsafe**, and hard to get it right. This will be more complicated to migrate efficiently.
//! The new `transmute_vec()` only works under very restricted conditions: the `mem::align_of()` and `mem::size_of()` between the
//! source and target element types must match. Otherwise, a full copy of the vector must be made.
//!
//! ```rust
//! # use safe_transmute::Error;
//! # #[cfg(feature = "alloc")]
//! # {
//! use safe_transmute::{transmute_vec, try_copy};
//!
//! let bytes = vec![0x00, 0x01, 0x12, 0x24, 0x00];
//! let words = try_copy!(transmute_vec::<_, u16>(bytes)); // !!! works, but will always copy
//!
//! assert_eq!(&*words, &[u16::from_be(0x0001), u16::from_be(0x1224)]);
//! # }
//! # Ok::<(), Error<u8, u16>>(())
//! ```
//!
//! Oftentimes, you'll just be avoiding vector transmutation entirely.
//!
//! In order to avoid copies, you can allocate a vector of the target type `T`, transmute a mutable slice of the vector
//! into the source data type `S`, and write the data in there directly. This still requires both `S` and `T` to be
//! trivially transmutable in order to be within the compiler's safety guarantees, though.

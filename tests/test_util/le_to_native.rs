extern crate core as le_to_native_core;


/// Verify: http://play.integer32.com/?gist=4cd795d6f45898c876a754cd3f3c2aaa&version=stable
#[allow(dead_code)]
trait LeToNative {
    fn le_to_native<T: Sized>(self) -> Self;
}

impl<'a> LeToNative for &'a mut [u8] {
    #[cfg(target_endian = "little")]
    fn le_to_native<T: Sized>(self) -> Self {
        self
    }

    #[cfg(target_endian = "big")]
    fn le_to_native<T: Sized>(self) -> Self {
        use le_to_native_core::mem::size_of;

        for elem in self.chunks_mut(size_of::<T>()) {
            elem.reverse();
        }
        self
    }
}

#[cfg(feature = "alloc")]
impl LeToNative for Vec<u8> {
    #[cfg(target_endian = "little")]
    fn le_to_native<T: Sized>(self) -> Self {
        self
    }

    #[cfg(target_endian = "big")]
    fn le_to_native<T: Sized>(mut self) -> Self {
        (&mut self[..]).le_to_native::<T>();
        self
    }
}

/// Test aids: rustc has started placing static byte arrays at odd offsets
macro_rules! impl_le_to_native_Le2NAl {
    ($nm:ident, $n:expr) => {
        #[repr(align(64))]
        #[allow(dead_code)]
        struct $nm([u8; $n]);

        impl le_to_native_core::borrow::Borrow<[u8]> for $nm {
            fn borrow(&self) -> &[u8] {
                &self.0
            }
        }

        impl le_to_native_core::borrow::BorrowMut<[u8]> for $nm {
            fn borrow_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }

        impl le_to_native_core::ops::Deref for $nm {
            type Target = [u8];

            fn deref(&self) -> &[u8] {
                &self.0
            }
        }

        impl le_to_native_core::ops::DerefMut for $nm {
            fn deref_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }

        impl LeToNative for $nm {
            #[cfg(target_endian = "little")]
            fn le_to_native<T: Sized>(self) -> Self {
                self
            }

            #[cfg(target_endian = "big")]
            fn le_to_native<T: Sized>(mut self) -> Self {
                (&mut self.0[..]).le_to_native::<T>();
                self
            }
        }
    }
}

impl_le_to_native_Le2NAl!(Le2NAl2, 2);
impl_le_to_native_Le2NAl!(Le2NAl4, 4);
impl_le_to_native_Le2NAl!(Le2NAl8, 8);

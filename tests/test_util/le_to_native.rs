#[cfg(target_endian = "big")]
use std::mem::size_of;


/// Verify: http://play.integer32.com/?gist=4cd795d6f45898c876a754cd3f3c2aaa&version=stable
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
        for elem in self.chunks_mut(size_of::<T>()) {
            elem.reverse();
        }
        self
    }
}

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

macro_rules! impl_le_to_native_array_u8 {
    ($n:expr) => {
        impl LeToNative for [u8; $n] {
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
    }
}

impl_le_to_native_array_u8!(1);
impl_le_to_native_array_u8!(2);
impl_le_to_native_array_u8!(3);
impl_le_to_native_array_u8!(4);
impl_le_to_native_array_u8!(5);
impl_le_to_native_array_u8!(6);
impl_le_to_native_array_u8!(7);
impl_le_to_native_array_u8!(8);
impl_le_to_native_array_u8!(9);
impl_le_to_native_array_u8!(10);
impl_le_to_native_array_u8!(11);
impl_le_to_native_array_u8!(12);
impl_le_to_native_array_u8!(13);
impl_le_to_native_array_u8!(14);
impl_le_to_native_array_u8!(15);
impl_le_to_native_array_u8!(16);
impl_le_to_native_array_u8!(17);
impl_le_to_native_array_u8!(18);
impl_le_to_native_array_u8!(19);
impl_le_to_native_array_u8!(20);
impl_le_to_native_array_u8!(21);
impl_le_to_native_array_u8!(22);
impl_le_to_native_array_u8!(23);
impl_le_to_native_array_u8!(24);
impl_le_to_native_array_u8!(25);
impl_le_to_native_array_u8!(26);
impl_le_to_native_array_u8!(27);
impl_le_to_native_array_u8!(28);
impl_le_to_native_array_u8!(29);
impl_le_to_native_array_u8!(30);
impl_le_to_native_array_u8!(31);
impl_le_to_native_array_u8!(32);

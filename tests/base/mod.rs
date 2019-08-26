mod transmute_many_permissive;
mod transmute_many_pedantic;
mod from_bytes_pedantic;
mod transmute_many;
mod from_bytes;


#[cfg(feature = "alloc")]
use safe_transmute::base;


#[cfg(feature = "alloc")]
#[test]
fn transmute_vec() {
    unsafe {
        assert_eq!(base::transmute_vec::<u16, i16>(vec![0x0100u16]), vec![0x0100i16]);
        assert_eq!(base::transmute_vec::<u16, i16>(vec![0x0100u16, 0x0200u16]), vec![0x0100i16, 0x0200i16]);
    }
}

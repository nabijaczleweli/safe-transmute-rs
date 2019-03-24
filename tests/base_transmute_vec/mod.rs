#![cfg(feature = "std")]

use safe_transmute::base::transmute_vec;


#[test]
fn just_enough() {
    unsafe {
        assert_eq!(transmute_vec::<u16, i16>(vec![0x0100u16]),
                   vec![0x0100i16]);
        assert_eq!(transmute_vec::<u16, i16>(vec![0x0100u16, 0x0200u16]),
                   vec![0x0100i16, 0x0200i16]);
    }
}

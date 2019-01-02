#![cfg(feature = "std")]


use safe_transmute::base::guarded_transmute_vec_permissive;
use self::super::LeToNative;


#[test]
fn too_short() {
    unsafe {
        assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![]), vec![]);
        assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00]), vec![]);
    }
}

#[test]
fn just_enough() {
    unsafe {
        assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00, 0x01].le_to_native::<u16>()), vec![0x0100u16]);
        assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
                   vec![0x0100u16, 0x0200u16]);
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00].le_to_native::<u16>()),
                   vec![0x0100u16]);
        assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02, 0x00].le_to_native::<u16>()),
                   vec![0x0100u16, 0x0200u16]);
        assert_eq!(guarded_transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00].le_to_native::<u16>()),
                   vec![0x0100u16, 0x0200u16, 0x0300u16]);
    }
}

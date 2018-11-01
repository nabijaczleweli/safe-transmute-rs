#![cfg(feature = "std")]


use safe_transmute::safe_transmute_vec_permissive;
use self::super::{LeToNative, aligned_vec};


#[test]
fn too_short() {
    assert_eq!(safe_transmute_vec_permissive::<u16>(aligned_vec::<u16>([].as_ref())), Ok(vec![]));
    assert_eq!(safe_transmute_vec_permissive::<u16>(aligned_vec::<u16>([0x00].as_ref())), Ok(vec![]));
}

#[test]
fn just_enough() {
    assert_eq!(safe_transmute_vec_permissive::<u16>(aligned_vec::<u16>([0x00, 0x01].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16]));
    assert_eq!(safe_transmute_vec_permissive::<u16>(aligned_vec::<u16>([0x00, 0x01, 0x00, 0x02].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16, 0x0200u16]));
}

#[test]
fn too_much() {
    assert_eq!(safe_transmute_vec_permissive::<u16>(aligned_vec::<u16>([0x00, 0x01, 0x00].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16]));
    assert_eq!(safe_transmute_vec_permissive::<u16>(aligned_vec::<u16>([0x00, 0x01, 0x00, 0x02, 0x00].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16, 0x0200u16]));
    assert_eq!(safe_transmute_vec_permissive::<u16>(aligned_vec::<u16>([0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16, 0x0200u16, 0x0300u16]));
}

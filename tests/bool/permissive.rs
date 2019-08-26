use safe_transmute::{Error, transmute_bool_permissive};


#[test]
fn too_short() {
    assert_eq!(transmute_bool_permissive([].as_ref()), Ok([].as_ref()));
}

#[test]
fn just_enough() {
    assert_eq!(transmute_bool_permissive([0x00, 0x01].as_ref()), Ok([false, true].as_ref()));
    assert_eq!(transmute_bool_permissive([0x00, 0x01, 0x00, 0x01].as_ref()),
               Ok([false, true, false, true].as_ref()));
}

#[test]
fn invalid_bytes() {
    assert_eq!(transmute_bool_permissive([0x00, 0x01, 0x02].as_ref()), Err(Error::InvalidValue));
    assert_eq!(transmute_bool_permissive([0x05, 0x01, 0x00].as_ref()), Err(Error::InvalidValue));
    assert_eq!(transmute_bool_permissive([0xFF].as_ref()), Err(Error::InvalidValue));
}

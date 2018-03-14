use safe_transmute::{ErrorReason, GuardError, Error, guarded_transmute_bool_pedantic};


#[test]
fn too_short() {
    assert_eq!(guarded_transmute_bool_pedantic([].as_ref()),
               Err(Error::Guard(GuardError {
                   required: 1,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(guarded_transmute_bool_pedantic([0x00, 0x01].as_ref()), Ok([false, true].into_iter().as_slice()));
    assert_eq!(guarded_transmute_bool_pedantic([0x01, 0x01, 0x00, 0x01].as_ref()),
               Ok([true, true, false, true].into_iter().as_slice()));
}

#[test]
fn invalid_bytes() {
    assert_eq!(guarded_transmute_bool_pedantic([0x00, 0x01, 0x02].as_ref()), Err(Error::InvalidValue));
    assert_eq!(guarded_transmute_bool_pedantic([0x05, 0x01, 0x00].as_ref()), Err(Error::InvalidValue));
    assert_eq!(guarded_transmute_bool_pedantic([0xFF].as_ref()), Err(Error::InvalidValue));
}

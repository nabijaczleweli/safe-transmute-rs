use safe_transmute::{ErrorReason, GuardError, Error, guarded_transmute_bool_vec_pedantic};


#[test]
fn too_short() {
    assert_eq!(guarded_transmute_bool_vec_pedantic(vec![]),
               Err(Error::Guard(GuardError {
                   required: 1,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(guarded_transmute_bool_vec_pedantic(vec![0x00, 0x01]), Ok(vec![false, true]));
    assert_eq!(guarded_transmute_bool_vec_pedantic(vec![0x00, 0x01, 0x00, 0x01]),
               Ok(vec![false, true, false, true]));
}

#[test]
fn invalid_bytes() {
    assert_eq!(guarded_transmute_bool_vec_pedantic(vec![0x00, 0x01, 0x02]), Err(Error::InvalidValue));
    assert_eq!(guarded_transmute_bool_vec_pedantic(vec![0x05, 0x01, 0x00]), Err(Error::InvalidValue));
    assert_eq!(guarded_transmute_bool_vec_pedantic(vec![0xFF]), Err(Error::InvalidValue));
}

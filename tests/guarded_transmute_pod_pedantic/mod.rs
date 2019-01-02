use safe_transmute::{Error, ErrorReason, GuardError, safe_transmute_to_bytes, safe_transmute_one_pedantic};


#[test]
fn too_short() {
    assert_eq!(safe_transmute_one_pedantic::<u32>(safe_transmute_to_bytes::<u32>(&[])),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 0,
                   reason: ErrorReason::InexactByteCount,
               })));
    assert_eq!(safe_transmute_one_pedantic::<u32>(&safe_transmute_to_bytes::<u32>(&[0])[..1]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 1,
                   reason: ErrorReason::InexactByteCount,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(safe_transmute_one_pedantic::<u32>(&safe_transmute_to_bytes::<u32>(&[0x0100_0000])),
               Ok(0x0100_0000));
}

#[test]
fn too_much() {
    assert_eq!(safe_transmute_one_pedantic::<u32>(&safe_transmute_to_bytes::<u32>(&[0x0100_0000, 0])[..5]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 5,
                   reason: ErrorReason::InexactByteCount,
               })));
}

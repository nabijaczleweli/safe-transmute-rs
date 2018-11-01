use safe_transmute::{ErrorReason, GuardError, Error, safe_transmute_many_pedantic, safe_transmute_to_bytes};


#[test]
fn too_short() {
    assert_eq!(safe_transmute_many_pedantic::<u16>(safe_transmute_to_bytes::<u16>(&[])),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
    assert_eq!(safe_transmute_many_pedantic::<u16>(&safe_transmute_to_bytes::<u16>(&[0])[..1]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x1000, 0x2000];
    let bytes = safe_transmute_to_bytes(words);
    assert_eq!(safe_transmute_many_pedantic::<u16>(&bytes[..2]), Ok(&words[..1]));
    assert_eq!(safe_transmute_many_pedantic::<u16>(bytes), Ok(words));
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x1000, 0x2000, 0x300];
    let bytes = safe_transmute_to_bytes(words);
    assert_eq!(safe_transmute_many_pedantic::<u16>(&bytes[..3]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 3,
                   reason: ErrorReason::InexactByteCount,
               })));
    assert_eq!(safe_transmute_many_pedantic::<u16>(&bytes[..5]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 5,
                   reason: ErrorReason::InexactByteCount,
               })));
}

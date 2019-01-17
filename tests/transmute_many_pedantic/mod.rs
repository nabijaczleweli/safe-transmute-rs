use safe_transmute::{ErrorReason, GuardError, Error, transmute_many_pedantic, transmute_to_bytes};


#[test]
fn too_short() {
    assert_eq!(transmute_many_pedantic::<u16>(transmute_to_bytes::<u16>(&[])),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
    assert_eq!(transmute_many_pedantic::<u16>(&transmute_to_bytes::<u16>(&[0])[..1]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x1000, 0x2000];
    let bytes = transmute_to_bytes(words);
    assert_eq!(transmute_many_pedantic::<u16>(&bytes[..2]), Ok(&words[..1]));
    assert_eq!(transmute_many_pedantic::<u16>(bytes), Ok(words));
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x1000, 0x2000, 0x300];
    let bytes = transmute_to_bytes(words);
    assert_eq!(transmute_many_pedantic::<u16>(&bytes[..3]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 3,
                   reason: ErrorReason::InexactByteCount,
               })));
    assert_eq!(transmute_many_pedantic::<u16>(&bytes[..5]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 5,
                   reason: ErrorReason::InexactByteCount,
               })));
}

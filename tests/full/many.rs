use safe_transmute::{SingleManyGuard, ErrorReason, GuardError, Error, transmute_to_bytes, transmute_many};


#[test]
fn too_short() {
    assert_eq!(transmute_many::<u16, SingleManyGuard>(transmute_to_bytes::<u16>(&[])),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
    assert_eq!(transmute_many::<u16, SingleManyGuard>(&transmute_to_bytes::<u16>(&[0])[..1]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x0100, 0x0200];
    let bytes = transmute_to_bytes(words);
    assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..2]), Ok(&words[..1]));
    assert_eq!(transmute_many::<u16, SingleManyGuard>(bytes), Ok(words));
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300, 0];
    let bytes = transmute_to_bytes(words);
    assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..3]), Ok(&words[..1]));
    assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..5]), Ok(&words[..2]));
    assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..7]), Ok(&words[..3]));
}

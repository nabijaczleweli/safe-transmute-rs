use safe_transmute::{guarded_transmute_pod_many_pedantic, guarded_transmute_to_bytes_pod_many, Error, ErrorReason, GuardError};

#[test]
fn too_short() {
    assert_eq!(
        guarded_transmute_pod_many_pedantic::<u16>(guarded_transmute_to_bytes_pod_many::<u16>(&[])),
        Err(Error::Guard(GuardError {
            required: 16 / 8,
            actual: 0,
            reason: ErrorReason::NotEnoughBytes,
        }))
    );
    assert_eq!(
        guarded_transmute_pod_many_pedantic::<u16>(&guarded_transmute_to_bytes_pod_many::<u16>(&[0])[..1]),
        Err(Error::Guard(GuardError {
            required: 16 / 8,
            actual: 1,
            reason: ErrorReason::NotEnoughBytes,
        }))
    );
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x01000, 0x02000];
    let bytes = guarded_transmute_to_bytes_pod_many(words);
    assert_eq!(
        guarded_transmute_pod_many_pedantic::<u16>(&bytes[..2]),
        Ok(&words[..1])
    );
    assert_eq!(
        guarded_transmute_pod_many_pedantic::<u16>(bytes),
        Ok(words)
    );
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x01000, 0x02000, 0x0300];
    let bytes = guarded_transmute_to_bytes_pod_many(words);
    assert_eq!(
        guarded_transmute_pod_many_pedantic::<u16>(&bytes[..3]),
        Err(Error::Guard(GuardError {
            required: 16 / 8,
            actual: 3,
            reason: ErrorReason::InexactByteCount,
        }))
    );
    assert_eq!(
        guarded_transmute_pod_many_pedantic::<u16>(&bytes[..5]),
        Err(Error::Guard(GuardError {
            required: 16 / 8,
            actual: 5,
            reason: ErrorReason::InexactByteCount,
        }))
    );
}

use safe_transmute::{Error, ErrorReason, GuardError, guarded_transmute_to_bytes_pod_many, guarded_transmute_pod_pedantic};


#[test]
fn too_short() {
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(guarded_transmute_to_bytes_pod_many::<u32>(&[])),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 0,
                   reason: ErrorReason::InexactByteCount,
               })));
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(&guarded_transmute_to_bytes_pod_many::<u32>(&[0])[..1]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 1,
                   reason: ErrorReason::InexactByteCount,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(&guarded_transmute_to_bytes_pod_many::<u32>(&[0x0100_0000])),
               Ok(0x0100_0000));
}

#[test]
fn too_much() {
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(&guarded_transmute_to_bytes_pod_many::<u32>(&[0x0100_0000, 0])[..5]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 5,
                   reason: ErrorReason::InexactByteCount,
               })));
}

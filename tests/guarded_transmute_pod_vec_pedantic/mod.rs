use safe_transmute::{ErrorReason, Error, guarded_transmute_pod_vec_pedantic};
use self::super::LeToNative;


#[test]
fn too_short() {
    assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![]),
               Err(Error {
                   required: 16 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               }));
    assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00]),
               Err(Error {
                   required: 16 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               }));
}

#[test]
fn just_enough() {
    assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01].le_to_native::<u16>()),
               Ok(vec![0x0100u16]));
    assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
               Ok(vec![0x0100u16, 0x0200u16]));
}

#[test]
fn too_much() {
    assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00]),
               Err(Error {
                   required: 16 / 8,
                   actual: 3,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(guarded_transmute_pod_vec_pedantic::<u16>(vec![0x00, 0x01, 0x00, 0x02, 0x00]),
               Err(Error {
                   required: 16 / 8,
                   actual: 5,
                   reason: ErrorReason::InexactByteCount,
               }));
}

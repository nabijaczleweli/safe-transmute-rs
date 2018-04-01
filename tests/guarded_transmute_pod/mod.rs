use safe_transmute::{ErrorReason, Error, GuardError, guarded_transmute_pod};
use self::super::LeToNative;


#[test]
fn too_short() {
    assert_eq!(guarded_transmute_pod::<u32>(&[]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
    assert_eq!(guarded_transmute_pod::<u32>(&[0x00]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()), Ok(0x01000000));
}

#[test]
fn too_much() {
    assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00].le_to_native::<u32>()), Ok(0x01000000));
    assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00].le_to_native::<u32>()),
               Ok(0x01000000));
    assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00].le_to_native::<u32>()),
               Ok(0x01000000));
    assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00].le_to_native::<u32>()),
               Ok(0x01000000));
    assert_eq!(guarded_transmute_pod::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00].le_to_native::<u32>()),
               Ok(0x01000000));
}

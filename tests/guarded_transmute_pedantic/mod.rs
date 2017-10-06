use safe_transmute::{ErrorReason, Error, guarded_transmute_pedantic};
use self::super::LeToNative;


#[test]
fn too_short() {
    unsafe {
        assert_eq!(guarded_transmute_pedantic::<u32>(&[]),
                   Err(Error {
                       required: 32 / 8,
                       actual: 0,
                       reason: ErrorReason::InexactByteCount,
                   }));
        assert_eq!(guarded_transmute_pedantic::<u32>(&[0x00]),
                   Err(Error {
                       required: 32 / 8,
                       actual: 1,
                       reason: ErrorReason::InexactByteCount,
                   }));
    }
}

#[test]
fn just_enough() {
    unsafe {
        assert_eq!(guarded_transmute_pedantic::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()), Ok(0x01000000));
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(guarded_transmute_pedantic::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00]),
                   Err(Error {
                       required: 32 / 8,
                       actual: 5,
                       reason: ErrorReason::InexactByteCount,
                   }));
    }
}

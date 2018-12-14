use safe_transmute::{ErrorReason, GuardError, Error, guarded_transmute};
use self::super::LeToNative;


#[test]
fn too_short() {
    unsafe {
        assert_eq!(guarded_transmute::<u32>(&[]),
                   Err(Error::Guard(GuardError {
                       required: 32 / 8,
                       actual: 0,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
        assert_eq!(guarded_transmute::<u32>(&[0x00]),
                   Err(Error::Guard(GuardError {
                       required: 32 / 8,
                       actual: 1,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
    }
}

#[test]
fn just_enough() {
    unsafe {
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()), Ok(0x0100_0000));
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00].le_to_native::<u32>()), Ok(0x0100_0000));
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00].le_to_native::<u32>()),
                   Ok(0x0100_0000));
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00].le_to_native::<u32>()),
                   Ok(0x0100_0000));
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00].le_to_native::<u32>()),
                   Ok(0x0100_0000));
        assert_eq!(guarded_transmute::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00].le_to_native::<u32>()),
                   Ok(0x0100_0000));
    }
}

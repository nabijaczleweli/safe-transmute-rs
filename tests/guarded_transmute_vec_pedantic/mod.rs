#![cfg(feature = "std")]


use safe_transmute::{PedanticGuard, ErrorReason, GuardError, Error};
use safe_transmute::base::guarded_transmute_vec;
use self::super::LeToNative;


#[test]
fn too_short() {
    unsafe {
        assert_eq!(guarded_transmute_vec::<u16, PedanticGuard>(vec![]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 0,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
        assert_eq!(guarded_transmute_vec::<u16, PedanticGuard>(vec![0x00]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 1,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
    }
}

#[test]
fn just_enough() {
    unsafe {
        assert_eq!(guarded_transmute_vec::<u16, PedanticGuard>(vec![0x00, 0x01].le_to_native::<u16>()),
                   Ok(vec![0x0100u16]));
        assert_eq!(guarded_transmute_vec::<u16, PedanticGuard>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
                   Ok(vec![0x0100u16, 0x0200u16]));
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(guarded_transmute_vec::<u16, PedanticGuard>(vec![0x00, 0x01, 0x00]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 3,
                       reason: ErrorReason::InexactByteCount,
                   })));
        assert_eq!(guarded_transmute_vec::<u16, PedanticGuard>(vec![0x00, 0x01, 0x00, 0x02, 0x00]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 5,
                       reason: ErrorReason::InexactByteCount,
                   })));
    }
}

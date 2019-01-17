#![cfg(feature = "std")]


use safe_transmute::{SingleManyGuard, ErrorReason, GuardError, Error, transmute_vec};
use self::super::{LeToNative, aligned_vec};

#[test]
fn too_short() {
    assert_eq!(transmute_vec::<u16, SingleManyGuard>(aligned_vec::<u16>([].as_ref())),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
    assert_eq!(transmute_vec::<u16, SingleManyGuard>(aligned_vec::<u16>([0x00].as_ref())),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(transmute_vec::<u16, SingleManyGuard>(aligned_vec::<u16>([0x00, 0x01].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16]));
    assert_eq!(transmute_vec::<u16, SingleManyGuard>(aligned_vec::<u16>([0x00, 0x01, 0x00, 0x02].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16, 0x0200u16]));
}

#[test]
fn too_much() {
    assert_eq!(transmute_vec::<u16, SingleManyGuard>(aligned_vec::<u16>([0x00, 0x01, 0x00].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16]));
    assert_eq!(transmute_vec::<u16, SingleManyGuard>(aligned_vec::<u16>([0x00, 0x01, 0x00, 0x02, 0x00].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16, 0x0200u16]));
    assert_eq!(transmute_vec::<u16, SingleManyGuard>(aligned_vec::<u16>([0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00].as_ref()).le_to_native::<u16>()),
               Ok(vec![0x0100u16, 0x0200u16, 0x0300u16]));
}

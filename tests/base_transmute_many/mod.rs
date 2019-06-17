use safe_transmute::{SingleManyGuard, ErrorReason, GuardError, Error, transmute_to_bytes, transmute_to_bytes_mut};
use safe_transmute::base::{transmute_many, transmute_many_mut};

#[test]
fn too_short() {
    unsafe {
        assert_eq!(transmute_many::<u16, SingleManyGuard>(&[]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 0,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
        assert_eq!(transmute_many::<u16, SingleManyGuard>(&[0x00]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 1,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
    }
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300];
    let bytes = transmute_to_bytes(words);

    unsafe {
        assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..2]),
                   Ok(&words[..1]));
        assert_eq!(transmute_many::<u16, SingleManyGuard>(bytes),
                   Ok(words));
    }
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300, 0];
    let bytes = transmute_to_bytes(words);

    unsafe {
        assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..3]),
                   Ok(&words[..1]));
        assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..5]),
                   Ok(&words[..2]));
        assert_eq!(transmute_many::<u16, SingleManyGuard>(&bytes[..7]),
                   Ok(&words[..3]));
    }
}

#[test]
fn too_short_mut() {
    unsafe {
        assert_eq!(transmute_many_mut::<u16, SingleManyGuard>(&mut []),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 0,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
        assert_eq!(transmute_many_mut::<u16, SingleManyGuard>(&mut [0x00]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 1,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
    }
}

#[test]
fn just_enough_mut() {
    let words: &mut [u16] = &mut [0x0100, 0x0200, 0x0300];
    let bytes = transmute_to_bytes_mut(words);
    // make an independent version of `words` for eq testing
    let words: &mut [u16] = &mut [0x0100, 0x0200, 0x0300];

    unsafe {
        assert_eq!(transmute_many_mut::<u16, SingleManyGuard>(&mut bytes[..2]),
                   Ok(&mut words[..1]));
        assert_eq!(transmute_many_mut::<u16, SingleManyGuard>(bytes),
                   Ok(words));
    }
}

#[test]
fn too_much_mut() {
    let words: &mut[u16] = &mut [0x0100, 0x0200, 0x0300, 0];
    let bytes = transmute_to_bytes_mut(words);
    // make an independent version of `words` for eq testing
    let words: &mut [u16] = &mut [0x0100, 0x0200, 0x0300];

    unsafe {
        assert_eq!(transmute_many_mut::<u16, SingleManyGuard>(&mut bytes[..3]),
                   Ok(&mut words[..1]));
        assert_eq!(transmute_many_mut::<u16, SingleManyGuard>(&mut bytes[..5]),
                   Ok(&mut words[..2]));
        assert_eq!(transmute_many_mut::<u16, SingleManyGuard>(&mut bytes[..7]),
                   Ok(&mut words[..3]));
    }
}

use safe_transmute::{ErrorReason, GuardError, Error, transmute_to_bytes};
use safe_transmute::base::from_bytes_pedantic;


#[test]
fn too_short() {
    unsafe {
        assert_eq!(from_bytes_pedantic::<u32>(&[]),
                   Err(Error::Guard(GuardError {
                       required: 32 / 8,
                       actual: 0,
                       reason: ErrorReason::InexactByteCount,
                   })));
        assert_eq!(from_bytes_pedantic::<u32>(&[0x00]),
                   Err(Error::Guard(GuardError {
                       required: 32 / 8,
                       actual: 1,
                       reason: ErrorReason::InexactByteCount,
                   })));
    }
}

#[test]
fn just_enough() {
    let word = [0x0100_0020];
    let bytes = transmute_to_bytes(&word);

    unsafe {
        assert_eq!(from_bytes_pedantic::<u32>(bytes), Ok(0x0100_0020));
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(from_bytes_pedantic::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00]),
                   Err(Error::Guard(GuardError {
                       required: 32 / 8,
                       actual: 5,
                       reason: ErrorReason::InexactByteCount,
                   })));
    }
}

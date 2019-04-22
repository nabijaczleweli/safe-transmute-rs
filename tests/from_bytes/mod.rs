use safe_transmute::{ErrorReason, GuardError, Error, transmute_to_bytes};
use safe_transmute::base::from_bytes;

#[test]
fn too_short() {
    unsafe {
        assert_eq!(from_bytes::<u32>(&[]),
                   Err(Error::Guard(GuardError {
                       required: 32 / 8,
                       actual: 0,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
        assert_eq!(from_bytes::<u32>(&[0x00]),
                   Err(Error::Guard(GuardError {
                       required: 32 / 8,
                       actual: 1,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
    }
}

#[test]
fn just_enough() {
    let word = [0x100_B0B0];
    let bytes = transmute_to_bytes(&word[..]);
    unsafe {
        assert_eq!(from_bytes::<u32>(bytes), Ok(0x0100_B0B0));
    }
}

#[test]
fn too_much() {
    let words = [0x100_C0C0, 0, 0, 0];
    let bytes = transmute_to_bytes(&words[..]);
    
    unsafe {
        assert_eq!(from_bytes::<u32>(&bytes[..5]), Ok(0x100_C0C0));
        assert_eq!(from_bytes::<u32>(&bytes[..6]), Ok(0x100_C0C0));
        assert_eq!(from_bytes::<u32>(&bytes[..7]), Ok(0x100_C0C0));
        assert_eq!(from_bytes::<u32>(&bytes[..8]), Ok(0x100_C0C0));
        assert_eq!(from_bytes::<u32>(&bytes[..9]), Ok(0x100_C0C0));
    }
}

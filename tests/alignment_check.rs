#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
extern crate core;
extern crate safe_transmute;

use safe_transmute::guarded_transmute_pod_many;
use safe_transmute::guarded_transmute_to_bytes_pod_many;
use safe_transmute::Error;

#[test]
fn unaligned_slicing_integers() {
    let words = [0x01FF, 0x02EE, 0x03DD, 0x04CC, 0x05BB, 0x06AA];
    let bytes = guarded_transmute_to_bytes_pod_many(&words);

    assert_eq!(guarded_transmute_pod_many::<u16>(bytes), Ok(words.as_ref()));
    assert_eq!(guarded_transmute_pod_many::<u16>(&bytes[1..]), Err(Error::Unaligned { offset: 1 }));
    assert_eq!(guarded_transmute_pod_many::<u16>(&bytes[2..]), Ok(&words[1..]));
    assert_eq!(guarded_transmute_pod_many::<u16>(&bytes[3..]), Err(Error::Unaligned { offset: 1 }));

    let words = [0x02EE01FF, 0x04CC03DD, 0x06AA05BB];
    let bytes = guarded_transmute_to_bytes_pod_many(&words);

    assert_eq!(guarded_transmute_pod_many::<u32>(bytes), Ok(words.as_ref()));
    assert_eq!(guarded_transmute_pod_many::<u32>(&bytes[1..]), Err(Error::Unaligned { offset: 3 }));
    assert_eq!(guarded_transmute_pod_many::<u32>(&bytes[2..]), Err(Error::Unaligned { offset: 2 }));
    assert_eq!(guarded_transmute_pod_many::<u32>(&bytes[3..]), Err(Error::Unaligned { offset: 1 }));
    assert_eq!(guarded_transmute_pod_many::<u32>(&bytes[4..]), Ok(&words[1..]));
    assert_eq!(guarded_transmute_pod_many::<u32>(&bytes[5..]), Err(Error::Unaligned { offset: 3 }));

    let words = [0x02EE01FF_04CC03DD];
    let bytes = guarded_transmute_to_bytes_pod_many(&words);
    assert_eq!(guarded_transmute_pod_many::<u64>(bytes), Ok(words.as_ref()));
    for i in 1..8 {
        assert_eq!(guarded_transmute_pod_many::<u64>(&bytes[i..]), Err(Error::Unaligned { offset: 8 - i }));
    }
}


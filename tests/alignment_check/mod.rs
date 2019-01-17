use safe_transmute::{transmute_many_permissive, transmute_to_bytes};
use safe_transmute::error::{UnalignedError, Error};


#[test]
fn unaligned_slicing_integers() {
    let words = [0x01FF, 0x02EE, 0x03DD, 0x04CC, 0x05BB, 0x06AA];
    let bytes = transmute_to_bytes(&words);

    assert_eq!(transmute_many_permissive::<u16>(bytes), Ok(words.as_ref()));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[1..]),
               Err(Error::Unaligned(UnalignedError { offset: 1 })));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[2..]), Ok(&words[1..]));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[3..]),
               Err(Error::Unaligned(UnalignedError { offset: 1 })));

    let words = [0x02EE_01FF, 0x04CC_03DD, 0x06AA_05BB];
    let bytes = transmute_to_bytes(&words);

    assert_eq!(transmute_many_permissive::<u32>(bytes), Ok(words.as_ref()));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[1..]),
               Err(Error::Unaligned(UnalignedError { offset: 3 })));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[2..]),
               Err(Error::Unaligned(UnalignedError { offset: 2 })));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[3..]),
               Err(Error::Unaligned(UnalignedError { offset: 1 })));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[4..]), Ok(&words[1..]));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[5..]),
               Err(Error::Unaligned(UnalignedError { offset: 3 })));

    let words = [0x02EE_01FF_04CC_03DD];
    let bytes = transmute_to_bytes(&words);
    assert_eq!(transmute_many_permissive::<u64>(bytes), Ok(words.as_ref()));
    for i in 1..8 {
        assert_eq!(transmute_many_permissive::<u64>(&bytes[i..]),
                   Err(Error::Unaligned(UnalignedError { offset: 8 - i })));
    }
}

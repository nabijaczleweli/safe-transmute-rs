use safe_transmute::{transmute_many_permissive, transmute_to_bytes};
use safe_transmute::error::{UnalignedError, Error};


#[test]
fn unaligned_slicing_integers() {
    let words = [0x01FF, 0x02EE, 0x03DD, 0x04CC, 0x05BB, 0x06AA];
    let bytes = transmute_to_bytes(&words);

    assert_eq!(transmute_many_permissive::<u16>(bytes), Ok(words.as_ref()));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[1..]),
               Err(Error::Unaligned(UnalignedError::new(1, &bytes[1..]))));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[2..]), Ok(&words[1..]));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[3..]),
               Err(Error::Unaligned(UnalignedError::new(1, &bytes[3..]))));

    let words = [0x02EE_01FF, 0x04CC_03DD, 0x06AA_05BB];
    let bytes = transmute_to_bytes(&words);

    assert_eq!(transmute_many_permissive::<u32>(bytes), Ok(words.as_ref()));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[1..]),
               Err(Error::Unaligned(UnalignedError::new(3, &bytes[1..]))));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[2..]),
               Err(Error::Unaligned(UnalignedError::new(2, &bytes[2..]))));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[3..]),
               Err(Error::Unaligned(UnalignedError::new(1, &bytes[3..]))));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[4..]), Ok(&words[1..]));
    assert_eq!(transmute_many_permissive::<u32>(&bytes[5..]),
               Err(Error::Unaligned(UnalignedError::new(3, &bytes[5..]))));

    let words = [0x02EE_01FF_04CC_03DD, 0x06EE_05FF_08CC_07DD];
    let bytes = transmute_to_bytes(&words);
    assert_eq!(transmute_many_permissive::<u64>(bytes), Ok(&words[..]));
    assert_eq!(transmute_many_permissive::<u64>(&bytes[8..]), Ok(&words[1..]));
    for i in 1..8 {
        let outcome = transmute_many_permissive::<u64>(&bytes[i..]);
        assert_eq!(outcome, Err(Error::Unaligned(UnalignedError::new(8 - i, &bytes[i..]))));
        #[cfg(feature = "std")]
        {
            let copied_data: Vec<_> = match outcome {
                Ok(_) => unreachable!(),
                Err(Error::Unaligned(e)) => e.copy(),
                Err(e) => panic!("Expected `UnalignedError`, got {}", e),
            };
            assert_eq!(copied_data.len(), 1);
            let number = u64::from(bytes[i])
                + (u64::from(bytes[i + 1]) << 8)
                + (u64::from(bytes[i + 2]) << 16)
                + (u64::from(bytes[i + 3]) << 24)
                + (u64::from(bytes[i + 4]) << 32)
                + (u64::from(bytes[i + 5]) << 40)
                + (u64::from(bytes[i + 6]) << 48)
                + (u64::from(bytes[i + 7]) << 56);
            assert_eq!(u64::to_le(copied_data[0]), number);
        }
    }
}

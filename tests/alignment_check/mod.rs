use safe_transmute::{transmute_many_permissive, transmute_to_bytes};
use safe_transmute::error::{UnalignedError, Error};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;


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

    // transmute aligned content
    assert_eq!(transmute_many_permissive::<u64>(bytes), Ok(&words[..]));
    assert_eq!(transmute_many_permissive::<u64>(&bytes[8..]), Ok(&words[1..]));

    // without try_copy!
    for i in 1..4 {
        // transmute unaligned content by copying
        let outcome = transmute_many_permissive::<u64>(&bytes[i..]);
        assert_eq!(outcome, Err(Error::Unaligned(UnalignedError::new(8 - i, &bytes[i..]))));

        #[cfg(feature = "alloc")]
        {
            let copied_data: Vec<_> = match outcome {
                Ok(_) => unreachable!(),
                Err(Error::Unaligned(e)) => e.copy(),
                Err(e) => panic!("Expected `UnalignedError`, got {}", e),
            };
            assert_eq!(copied_data.len(), 1);

            let expected_word: u64 = (0..8)
                .map(|k| u64::from(bytes[i + k]) << (8 * k))
                .sum();
            assert_eq!(u64::to_le(copied_data[0]), expected_word);
        }
    }

    // with try_copy!
    #[cfg(feature = "alloc")]
    unaligned_slicing_integers_with_try_copy(bytes).unwrap();
}

#[cfg(feature = "alloc")]
fn unaligned_slicing_integers_with_try_copy<'a>(bytes: &'a [u8]) -> Result<(), Error<'a, u8, u64>> {
    for i in 4..8 {
        // transmute unaligned content by copying
        let outcome = transmute_many_permissive::<u64>(&bytes[i..]);
        assert_eq!(outcome, Err(Error::Unaligned(UnalignedError::new(8 - i, &bytes[i..]))));

        let copied_data = try_copy!(outcome);
        assert_eq!(copied_data.len(), 1);

        let expected_word: u64 = (0..8)
            .map(|k| u64::from(bytes[i + k]) << (8 * k))
            .sum();
        assert_eq!(u64::to_le(copied_data[0]), expected_word);
    }

    Ok(())
}

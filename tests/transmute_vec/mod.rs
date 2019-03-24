#![cfg(feature = "std")]

use safe_transmute::error::IncompatibleVecTargetError;
use safe_transmute::{transmute_vec, Error};

#[test]
fn bad_size() {
    assert_eq!(
        transmute_vec::<u16, [u16; 2]>(vec![]),
        Err(Error::IncompatibleVecTarget(IncompatibleVecTargetError::new(vec![])))
    );
    assert_eq!(
        transmute_vec::<u16, [u8; 4]>(vec![1, 2, 3]),
        Err(Error::IncompatibleVecTarget(IncompatibleVecTargetError::new(vec![1, 2, 3])))
    );
}

#[test]
fn just_enough() {
    assert_eq!(
        transmute_vec::<u8, i8>(vec![0x00, 0x01]),
        Ok(vec![0x00i8, 0x01i8])
    );
    assert_eq!(
        transmute_vec::<u16, i16>(vec![0x0100u16, 0x0200u16]),
        Ok(vec![0x0100i16, 0x0200i16])
    );
}

#[test]
fn bad_alignment() {
    assert_eq!(
        transmute_vec::<u16, [u8; 2]>(vec![8, 8, 8]),
        Err(Error::IncompatibleVecTarget(IncompatibleVecTargetError::new(vec![8, 8, 8])))
    );
    assert_eq!(
        transmute_vec::<u64, [u16; 4]>(vec![3, 2, 1]),
        Err(Error::IncompatibleVecTarget(IncompatibleVecTargetError::new(vec![3, 2, 1])))
    );
}

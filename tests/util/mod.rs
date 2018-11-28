use safe_transmute::util;
use core::{f32, f64};
use core::mem::align_of;


#[test]
fn designalise_f32() {
    assert_eq!(util::designalise_f32(12.34125121), 12.34125121);
    assert!(util::designalise_f32(f32::NAN).is_nan());
    // I'm not quite sure how to make an sNaN to test this so...
}

#[test]
fn designalise_f64() {
    assert_eq!(util::designalise_f64(12.34125121), 12.34125121);
    assert!(util::designalise_f64(f64::NAN).is_nan());
    // I'm not quite sure how to make an sNaN to test this, either
}

#[test]
fn smoke_check_alignment_from_4() {
    let x: [i32; 5] = [0x5555_5555; 5];
    assert_eq!(align_of::<[i32; 5]>(), 4);
    assert_eq!(util::check_alignment::<_, u8>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, i8>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, u16>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, i16>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, u32>(&x[..]), Ok(()));
}

#[test]
fn smoke_check_alignment_from_8() {
    let x: [i64; 5] = [0x5555_5555_5555_5555; 5];
    assert_eq!(align_of::<[i64; 5]>(), 8);
    assert_eq!(util::check_alignment::<_, u8>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, i8>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, u16>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, i16>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, u32>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, i32>(&x[..]), Ok(()));
    assert_eq!(util::check_alignment::<_, u64>(&x[..]), Ok(()));
}

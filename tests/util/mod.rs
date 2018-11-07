use safe_transmute::util;
use core::{f32, f64};


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

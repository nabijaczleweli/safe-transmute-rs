use safe_transmute::{transmute_many_permissive, transmute_to_bytes};


#[test]
fn too_short() {
    assert_eq!(transmute_many_permissive::<u16>(transmute_to_bytes::<u16>(&[])), Ok([].as_ref()));
    assert_eq!(transmute_many_permissive::<u16>(&transmute_to_bytes::<u16>(&[0])[..1]),
               Ok([].as_ref()));
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x0100u16, 0x0200u16];
    let bytes = transmute_to_bytes(words);
    assert_eq!(transmute_many_permissive::<u16>(&bytes[..2]), Ok(&words[..1]));
    assert_eq!(transmute_many_permissive::<u16>(bytes), Ok(words));
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300, 0];
    let bytes = transmute_to_bytes(words);
    assert_eq!(transmute_many_permissive::<u16>(&bytes[..3]), Ok(&words[..1]));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[..5]), Ok(&words[..2]));
    assert_eq!(transmute_many_permissive::<u16>(&bytes[..7]), Ok(&words[..3]));
}

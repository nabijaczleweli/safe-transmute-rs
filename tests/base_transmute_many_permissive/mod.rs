use safe_transmute::base::transmute_many_permissive;
use self::super::LeToNative;


#[test]
fn too_short() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300];
    let bytes = transmute_to_bytes(words);

    unsafe {
        assert_eq!(transmute_many::<u16, PermissiveGuard>(&[]), Ok(&[]));
        assert_eq!(transmute_many::<u16, PermissiveGuard>(&bytes[..1]),
                   Ok(words));
    }
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300];
    let bytes = transmute_to_bytes(words);

    unsafe {
        assert_eq!(transmute_many::<u16, PermissiveGuard>(&bytes[..2]),
                   Ok(&words[..1]));
        assert_eq!(transmute_many::<u16, PermissiveGuard>(bytes),
                   Ok(words));
    }
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300, 0];
    let bytes = transmute_to_bytes(words);

    unsafe {
        assert_eq!(transmute_many::<u16, PermissiveGuard>(&bytes[..3]),
                   Ok(&words[..1]));
        assert_eq!(transmute_many::<u16, PermissiveGuard>(&bytes[..5]),
                   Ok(&words[..2]));
        assert_eq!(transmute_many::<u16, PermissiveGuard>(&bytes[..7]),
                   Ok(&words[..3]));
    }
}

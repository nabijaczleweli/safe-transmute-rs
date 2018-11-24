use safe_transmute::{guarded_transmute_pod_many_permissive, guarded_transmute_to_bytes_pod_many};

#[test]
fn too_short() {
    assert_eq!(
        guarded_transmute_pod_many_permissive::<u16>(guarded_transmute_to_bytes_pod_many::<u16>(&[])),
        Ok([].as_ref())
    );
    assert_eq!(
        guarded_transmute_pod_many_permissive::<u16>(&guarded_transmute_to_bytes_pod_many::<u16>(&[0])[..1]),
        Ok([].as_ref())
    );
}

#[test]
fn just_enough() {
    let words: &[u16] = &[0x0100u16, 0x0200u16];
    let bytes = guarded_transmute_to_bytes_pod_many(words);
    assert_eq!(guarded_transmute_pod_many_permissive::<u16>(&bytes[..2]), Ok(&words[..1]));
    assert_eq!(guarded_transmute_pod_many_permissive::<u16>(bytes), Ok(words));
}

#[test]
fn too_much() {
    let words: &[u16] = &[0x0100, 0x0200, 0x0300, 0];
    let bytes = guarded_transmute_to_bytes_pod_many(words);
    assert_eq!(guarded_transmute_pod_many_permissive::<u16>(&bytes[..3]), Ok(&words[..1]));
    assert_eq!(guarded_transmute_pod_many_permissive::<u16>(&bytes[..5]), Ok(&words[..2]));
    assert_eq!(guarded_transmute_pod_many_permissive::<u16>(&bytes[..7]), Ok(&words[..3]));
}

use self::super::LeToNative;
use safe_transmute::guarded_transmute_pod_many_permissive;

#[test]
fn too_short() {
    assert_eq!(guarded_transmute_pod_many_permissive::<u16>(&[]), Ok([].as_ref()));
    assert_eq!(guarded_transmute_pod_many_permissive::<u16>(&[0x00]), Ok([].as_ref()));
}

#[test]
fn just_enough() {
    assert_eq!(
        guarded_transmute_pod_many_permissive::<u16>(&[0x00, 0x01].le_to_native::<u16>()),
        Ok([0x0100u16].as_ref())
    );
    assert_eq!(
        guarded_transmute_pod_many_permissive::<u16>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
        Ok([0x0100u16, 0x0200u16].as_ref())
    );
}

#[test]
fn too_much() {
    assert_eq!(
        guarded_transmute_pod_many_permissive::<u16>(&[0x00, 0x01, 0x00].le_to_native::<u16>()),
        Ok([0x0100u16].as_ref())
    );
    assert_eq!(
        guarded_transmute_pod_many_permissive::<u16>(&[0x00, 0x01, 0x00, 0x02, 0x00].le_to_native::<u16>()),
        Ok([0x0100u16, 0x0200u16].as_ref())
    );
    assert_eq!(
        guarded_transmute_pod_many_permissive::<u16>(&[0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00].le_to_native::<u16>()),
        Ok([0x0100u16, 0x0200u16, 0x0300u16].as_ref())
    );
}

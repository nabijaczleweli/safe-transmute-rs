use safe_transmute::guard::{AllOrNothingGuard, SingleValueGuard, PermissiveGuard, SingleManyGuard, PedanticGuard, Guard};
use safe_transmute::error::{ErrorReason, GuardError};


#[test]
fn single_value_guard() {
    assert_eq!(SingleValueGuard::check::<()>(&[]), Ok(()));
    assert_eq!(SingleValueGuard::check::<()>(&[0]),
               Err(GuardError {
                   required: 0,
                   actual: 1,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(SingleValueGuard::check::<()>(&[0, 1]),
               Err(GuardError {
                   required: 0,
                   actual: 2,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(SingleValueGuard::check::<()>(&[0, 1, 2]),
               Err(GuardError {
                   required: 0,
                   actual: 3,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(SingleValueGuard::check::<()>(&[0, 1, 2, 3]),
               Err(GuardError {
                   required: 0,
                   actual: 4,
                   reason: ErrorReason::InexactByteCount,
               }));
}

#[test]
fn pedantic_guard() {
    assert_eq!(PedanticGuard::check::<()>(&[]), Ok(()));
    assert_eq!(PedanticGuard::check::<()>(&[0]),
               Err(GuardError {
                   required: 0,
                   actual: 1,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(PedanticGuard::check::<()>(&[0, 1]),
               Err(GuardError {
                   required: 0,
                   actual: 2,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(PedanticGuard::check::<()>(&[0, 1, 2]),
               Err(GuardError {
                   required: 0,
                   actual: 3,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(PedanticGuard::check::<()>(&[0, 1, 2, 3]),
               Err(GuardError {
                   required: 0,
                   actual: 4,
                   reason: ErrorReason::InexactByteCount,
               }));
}

#[test]
fn all_or_nothing_guard() {
    assert_eq!(AllOrNothingGuard::check::<()>(&[]), Ok(()));
    assert_eq!(AllOrNothingGuard::check::<()>(&[0]),
               Err(GuardError {
                   required: 0,
                   actual: 1,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(AllOrNothingGuard::check::<()>(&[0, 1]),
               Err(GuardError {
                   required: 0,
                   actual: 2,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(AllOrNothingGuard::check::<()>(&[0, 1, 2]),
               Err(GuardError {
                   required: 0,
                   actual: 3,
                   reason: ErrorReason::InexactByteCount,
               }));
    assert_eq!(AllOrNothingGuard::check::<()>(&[0, 1, 2, 3]),
               Err(GuardError {
                   required: 0,
                   actual: 4,
                   reason: ErrorReason::InexactByteCount,
               }));
}

#[test]
fn single_many_guard() {
    assert_eq!(SingleManyGuard::check::<()>(&[]), Ok(()));
    assert_eq!(SingleManyGuard::check::<()>(&[0]), Ok(()));
    assert_eq!(SingleManyGuard::check::<()>(&[0, 1]), Ok(()));
    assert_eq!(SingleManyGuard::check::<()>(&[0, 1, 2]), Ok(()));
    assert_eq!(SingleManyGuard::check::<()>(&[0, 1, 2, 3]), Ok(()));
}

#[test]
fn permissive_guard() {
    assert_eq!(PermissiveGuard::check::<()>(&[]), Ok(()));
    assert_eq!(PermissiveGuard::check::<()>(&[0]), Ok(()));
    assert_eq!(PermissiveGuard::check::<()>(&[0, 1]), Ok(()));
    assert_eq!(PermissiveGuard::check::<()>(&[0, 1, 2]), Ok(()));
    assert_eq!(PermissiveGuard::check::<()>(&[0, 1, 2, 3]), Ok(()));
}

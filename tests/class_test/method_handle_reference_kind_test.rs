extern crate div;
use div::class::MethodHandleReferenceKind;

#[test]
fn test_method_handle_reference_kind() {
    assert_eq!(1, MethodHandleReferenceKind::GetField as u8);
    assert_eq!(2, MethodHandleReferenceKind::GetStatic as u8);
    assert_eq!(3, MethodHandleReferenceKind::PutField as u8);
    assert_eq!(4, MethodHandleReferenceKind::PutStatic as u8);
    assert_eq!(5, MethodHandleReferenceKind::InvokeVirtual as u8);
    assert_eq!(6, MethodHandleReferenceKind::InvokeStatic as u8);
    assert_eq!(7, MethodHandleReferenceKind::InvokeSpecial as u8);
    assert_eq!(8, MethodHandleReferenceKind::NewInvokeSpecial as u8);
    assert_eq!(9, MethodHandleReferenceKind::InvokeInterface as u8);
}

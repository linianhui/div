extern crate div;
use div::class::ConstantMethodHandle;
use div::class::ConstantTag;
use div::class::MethodHandleReferenceKind;

#[test]
fn test_constant_method_handle() {
    let constant_method_handle = ConstantMethodHandle::from(&[0x02, 0x00, 0x13]);
    assert_eq!(ConstantTag::MethodHandle, constant_method_handle.tag);
    assert_eq!(
        MethodHandleReferenceKind::GetStatic,
        constant_method_handle.reference_kind
    );
    assert_eq!(19, constant_method_handle.reference_index);
}

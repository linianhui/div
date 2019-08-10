extern crate div;
use div::class::ConstantInterfaceMethodRef;
use div::class::ConstantTag;

#[test]
fn test_constant_interface_method_ref() {
    let constant_interface_method_ref = ConstantInterfaceMethodRef::from(&[0x00, 0x14, 0x00, 0x15]);
    assert_eq!(
        ConstantTag::InterfaceMethodRef,
        constant_interface_method_ref.tag
    );
    assert_eq!(20, constant_interface_method_ref.class_index);
    assert_eq!(21, constant_interface_method_ref.name_and_type_index);
}

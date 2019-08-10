extern crate div;
use div::class::ConstantMethodRef;
use div::class::ConstantTag;

#[test]
fn test_constant_method_ref() {
    let constant_method_ref = ConstantMethodRef::from(&[0x00, 0x12, 0x00, 0x13]);
    assert_eq!(ConstantTag::MethodRef, constant_method_ref.tag);
    assert_eq!(18, constant_method_ref.class_index);
    assert_eq!(19, constant_method_ref.name_and_type_index);
}

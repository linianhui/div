extern crate div;
use div::class::ConstantFieldRef;
use div::class::ConstantTag;

#[test]
fn test_constant_field_ref() {
    let constant_field_ref = ConstantFieldRef::from(&[0x00, 0x10, 0x00, 0x11]);
    assert_eq!(ConstantTag::FieldRef, constant_field_ref.tag);
    assert_eq!(16, constant_field_ref.class_index);
    assert_eq!(17, constant_field_ref.name_and_type_index);
}

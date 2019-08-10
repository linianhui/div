extern crate div;
use div::class::ConstantNameAndType;
use div::class::ConstantTag;

#[test]
fn test_constant_name_and_type() {
    let constant_name_and_type = ConstantNameAndType::from(&[0x00, 0x12, 0x00, 0x13]);
    assert_eq!(ConstantTag::NameAndType, constant_name_and_type.tag);
    assert_eq!(18, constant_name_and_type.name_index);
    assert_eq!(19, constant_name_and_type.descriptor_index);
}

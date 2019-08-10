extern crate div;
use div::class::ConstantMethodType;
use div::class::ConstantTag;

#[test]
fn test_constant_method_type() {
    let constant_method_type = ConstantMethodType::from(&[0x00, 0x15]);
    assert_eq!(ConstantTag::MethodType, constant_method_type.tag);
    assert_eq!(21, constant_method_type.descriptor_index);
}

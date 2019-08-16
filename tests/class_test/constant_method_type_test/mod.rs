extern crate div;
use div::class::ConstantMethodType;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_method_type() {
    let mut u8_reader = U8Reader::from(&[0x00, 0x15]);
    let constant_method_type = ConstantMethodType::from(&mut u8_reader);
    assert_eq!(ConstantTag::MethodType, constant_method_type.tag);
    assert_eq!(21, constant_method_type.descriptor_index);
}

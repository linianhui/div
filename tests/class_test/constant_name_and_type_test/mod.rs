extern crate div;
use div::class::ConstantNameAndType;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_name_and_type() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x12, 0x00, 0x13]);
    let constant_name_and_type = ConstantNameAndType::new(&mut u8_reader);
    assert_eq!(ConstantTag::NameAndType, constant_name_and_type.tag);
    assert_eq!(18, constant_name_and_type.name_index);
    assert_eq!(19, constant_name_and_type.descriptor_index);
}

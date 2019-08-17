extern crate div;
use div::class::ConstantClass;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_class() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x15]);
    let constant_class = ConstantClass::new(&mut u8_reader);
    assert_eq!(ConstantTag::Class, constant_class.tag);
    assert_eq!(21, constant_class.name_index);
    assert_eq!(2, u8_reader.offset);
}

extern crate div;
use div::class::ConstantString;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_string() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x15]);
    let constant_string = ConstantString::new(&mut u8_reader);
    assert_eq!(ConstantTag::String, constant_string.tag);
    assert_eq!(21, constant_string.string_index);
}

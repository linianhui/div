extern crate div;
use div::class::ConstantInteger;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_integer() {
    let mut u8_reader = U8Reader::new(&[0x12, 0x34, 0x56, 0x78]);
    let constant_integer = ConstantInteger::new(&mut u8_reader);
    assert_eq!(ConstantTag::Integer, constant_integer.tag);
    assert_eq!(0x12_34_56_78, constant_integer.bytes);
    assert_eq!(4, u8_reader.offset);
}

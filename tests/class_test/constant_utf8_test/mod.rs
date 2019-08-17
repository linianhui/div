extern crate div;
use div::class::ConstantTag;
use div::class::ConstantUtf8;
use div::class::U8Reader;

#[test]
fn test_constant_utf8() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x06, 0x3C, 0x69, 0x6E, 0x69, 0x74, 0x3E]);
    let constant_utf8 = ConstantUtf8::new(&mut u8_reader);
    assert_eq!(ConstantTag::Utf8, constant_utf8.tag);
    assert_eq!(6, constant_utf8.length);
    assert_eq!(8, u8_reader.offset);
}

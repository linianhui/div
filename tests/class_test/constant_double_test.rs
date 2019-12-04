extern crate div;
use div::class::ConstantDouble;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_double() {
    let mut u8_reader = U8Reader::new(&[0x40, 0x09, 0x21, 0xFB, 0x54, 0x43, 0xD1, 0xA8]);
    let constant_double = ConstantDouble::new(&mut u8_reader);
    assert_eq!(ConstantTag::Double, constant_double.tag);
    assert_eq!(3.1415_9265_3579_3978_f64, constant_double.value);
    assert_eq!(8, u8_reader.position);
}

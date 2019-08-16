extern crate div;
use div::class::ConstantFloat;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_float() {
    let mut u8_reader = U8Reader::from(&[0x40, 0x49, 0x0F, 0xDB]);
    let constant_float = ConstantFloat::from(&mut u8_reader);
    assert_eq!(ConstantTag::Float, constant_float.tag);
    assert_eq!(3.1415_9265_f32, constant_float.bytes);
}

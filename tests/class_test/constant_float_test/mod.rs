extern crate div;
use div::class::ConstantFloat;
use div::class::ConstantTag;

#[test]
fn test_constant_float() {
    let constant_float = ConstantFloat::from(&[0x40, 0x49, 0x0F, 0xDB]);
    assert_eq!(ConstantTag::Float, constant_float.tag);
    assert_eq!(3.1415_9265_f32, constant_float.bytes);
}

extern crate div;
use div::class::ConstantFloat;
use div::class::ConstantKind;

#[test]
fn test_constant_float() {
    let constant_float = ConstantFloat::from(&[0x40, 0x49, 0x0F, 0xDB]);
    assert_eq!(ConstantKind::Float, constant_float.kind);
    assert_eq!(3.1415_9265_f32, constant_float.value);
}

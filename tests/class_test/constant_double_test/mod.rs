extern crate div;
use div::class::ConstantDouble;
use div::class::ConstantKind;

#[test]
fn test_constant_double() {
    let constant_double = ConstantDouble::from(&[0x40, 0x09, 0x21, 0xFB, 0x54, 0x43, 0xD1, 0xA8]);
    assert_eq!(ConstantKind::Double, constant_double.kind);
    assert_eq!(3.1415_9265_3579_3978_f64, constant_double.value);
}

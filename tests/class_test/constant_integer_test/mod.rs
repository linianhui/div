extern crate div;
use div::class::ConstantInteger;
use div::class::ConstantTag;

#[test]
fn test_constant_integer() {
    let constant_integer = ConstantInteger::from(&[0x12, 0x34, 0x56, 0x78]);
    assert_eq!(ConstantTag::Integer, constant_integer.tag);
    assert_eq!(0x12_34_56_78, constant_integer.bytes);
}

extern crate div;
use div::class::ConstantClass;
use div::class::ConstantTag;

#[test]
fn test_constant_class() {
    let constant_class = ConstantClass::from(&[0x00, 0x15]);
    assert_eq!(ConstantTag::Class, constant_class.tag);
    assert_eq!(21, constant_class.name_index);
}

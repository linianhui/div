extern crate div;
use div::class::ConstantString;
use div::class::ConstantTag;

#[test]
fn test_constant_string() {
    let constant_string = ConstantString::from(&[0x00, 0x15]);
    assert_eq!(ConstantTag::String, constant_string.tag);
    assert_eq!(21, constant_string.string_index);
}

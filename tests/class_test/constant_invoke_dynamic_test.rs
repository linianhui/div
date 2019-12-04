extern crate div;
use div::class::ConstantInvokeDynamic;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_invoke_dynamic() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x12, 0x00, 0x13]);
    let constant_invoke_dynamic = ConstantInvokeDynamic::new(&mut u8_reader);
    assert_eq!(ConstantTag::InvokeDynamic, constant_invoke_dynamic.tag);
    assert_eq!(18, constant_invoke_dynamic.bootstrap_method_attr_index);
    assert_eq!(19, constant_invoke_dynamic.name_and_type_index);
    assert_eq!(4, u8_reader.position);
}

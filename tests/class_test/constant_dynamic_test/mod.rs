extern crate div;
use div::class::ConstantDynamic;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_dynamic() {
    let mut u8_reader = U8Reader::from(&[0x00, 0x12, 0x00, 0x13]);
    let constant_dynamic = ConstantDynamic::from(&mut u8_reader);
    assert_eq!(ConstantTag::Dynamic, constant_dynamic.tag);
    assert_eq!(18, constant_dynamic.bootstrap_method_attr_index);
    assert_eq!(19, constant_dynamic.name_and_type_index);
}

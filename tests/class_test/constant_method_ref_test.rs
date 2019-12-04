extern crate div;
use div::class::ConstantMethodRef;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_method_ref() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x12, 0x00, 0x13]);
    let constant_method_ref = ConstantMethodRef::new(&mut u8_reader);
    assert_eq!(ConstantTag::MethodRef, constant_method_ref.tag);
    assert_eq!(18, constant_method_ref.class_index);
    assert_eq!(19, constant_method_ref.name_and_type_index);
    assert_eq!(4, u8_reader.position);
}

extern crate div;
use div::class::ConstantFieldRef;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_field_ref() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x10, 0x00, 0x11]);
    let constant_field_ref = ConstantFieldRef::new(&mut u8_reader);
    assert_eq!(ConstantTag::FieldRef, constant_field_ref.tag);
    assert_eq!(16, constant_field_ref.class_index);
    assert_eq!(17, constant_field_ref.name_and_type_index);
    assert_eq!(4, u8_reader.offset);
}

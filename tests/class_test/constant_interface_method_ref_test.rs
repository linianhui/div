extern crate div;
use div::class::ConstantInterfaceMethodRef;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_interface_method_ref() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x14, 0x00, 0x15]);
    let constant_interface_method_ref = ConstantInterfaceMethodRef::new(&mut u8_reader);
    assert_eq!(
        ConstantTag::InterfaceMethodRef,
        constant_interface_method_ref.tag
    );
    assert_eq!(20, constant_interface_method_ref.class_index);
    assert_eq!(21, constant_interface_method_ref.name_and_type_index);
    assert_eq!(4, u8_reader.position);
}

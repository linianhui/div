extern crate div;
use div::class::ConstantMethodHandle;
use div::class::ConstantTag;
use div::class::MethodHandleReferenceKind;
use div::class::U8Reader;

#[test]
fn test_constant_method_handle() {
    let mut u8_reader = U8Reader::new(&[0x02, 0x00, 0x13]);
    let constant_method_handle = ConstantMethodHandle::new(&mut u8_reader);
    assert_eq!(ConstantTag::MethodHandle, constant_method_handle.tag);
    assert_eq!(
        MethodHandleReferenceKind::GetStatic,
        constant_method_handle.reference_kind
    );
    assert_eq!(19, constant_method_handle.reference_index);
    assert_eq!(3, u8_reader.offset);
}

extern crate div;
use div::class::FieldAccessFlags;

#[test]
fn test_field_access_flags() {
    assert_eq!(0x_00_01, FieldAccessFlags::Public as u16);
    assert_eq!(0x_00_02, FieldAccessFlags::Private as u16);
    assert_eq!(0x_00_04, FieldAccessFlags::Protected as u16);
    assert_eq!(0x_00_08, FieldAccessFlags::Static as u16);
    assert_eq!(0x_00_10, FieldAccessFlags::Final as u16);
    assert_eq!(0x_00_40, FieldAccessFlags::Volatile as u16);
    assert_eq!(0x_00_80, FieldAccessFlags::Transient as u16);
    assert_eq!(0x_10_00, FieldAccessFlags::Synthetic as u16);
    assert_eq!(0x_40_00, FieldAccessFlags::Enum as u16);
}

extern crate div;
use div::class::ClassAccessFlags;

#[test]
fn test_class_access_flags() {
    assert_eq!(0x_00_01, ClassAccessFlags::Public as u16);
    assert_eq!(0x_00_10, ClassAccessFlags::Final as u16);
    assert_eq!(0x_00_20, ClassAccessFlags::Super as u16);
    assert_eq!(0x_02_00, ClassAccessFlags::Interface as u16);
    assert_eq!(0x_04_00, ClassAccessFlags::Abstract as u16);
    assert_eq!(0x_10_00, ClassAccessFlags::Synthetic as u16);
    assert_eq!(0x_20_00, ClassAccessFlags::Annotation as u16);
    assert_eq!(0x_40_00, ClassAccessFlags::Enum as u16);
    assert_eq!(0x_80_00, ClassAccessFlags::Module as u16);
}

extern crate div;
use div::class::MethodAccessFlags;

#[test]
fn test_method_access_flags() {
    assert_eq!(0x_00_01, MethodAccessFlags::Public as u16);
    assert_eq!(0x_00_02, MethodAccessFlags::Private as u16);
    assert_eq!(0x_00_04, MethodAccessFlags::Protected as u16);
    assert_eq!(0x_00_08, MethodAccessFlags::Static as u16);
    assert_eq!(0x_00_10, MethodAccessFlags::Final as u16);
    assert_eq!(0x_00_20, MethodAccessFlags::Synchronized as u16);
    assert_eq!(0x_00_40, MethodAccessFlags::Bridge as u16);
    assert_eq!(0x_00_80, MethodAccessFlags::Varargs as u16);
    assert_eq!(0x_01_00, MethodAccessFlags::Native as u16);
    assert_eq!(0x_04_00, MethodAccessFlags::Abstract as u16);
    assert_eq!(0x_08_00, MethodAccessFlags::Strict as u16);
    assert_eq!(0x_10_00, MethodAccessFlags::Synthetic as u16);
}

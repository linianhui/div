extern crate div;
use div::class::ConstantKind;

#[test]
fn test_constant_kind() {
    assert_eq!(1, ConstantKind::Utf8 as u8);
    assert_eq!(3, ConstantKind::Integer as u8);
    assert_eq!(4, ConstantKind::Float as u8);
    assert_eq!(5, ConstantKind::Long as u8);
    assert_eq!(6, ConstantKind::Double as u8);
    assert_eq!(7, ConstantKind::Class as u8);
    assert_eq!(8, ConstantKind::String as u8);
    assert_eq!(9, ConstantKind::FieldRef as u8);
    assert_eq!(10, ConstantKind::MethodRef as u8);
    assert_eq!(11, ConstantKind::InterfaceMethodRef as u8);
    assert_eq!(12, ConstantKind::NameAndType as u8);
    assert_eq!(15, ConstantKind::MethodHandle as u8);
    assert_eq!(16, ConstantKind::MethodType as u8);
    assert_eq!(17, ConstantKind::Dynamic as u8);
    assert_eq!(18, ConstantKind::InvokeDynamic as u8);
    assert_eq!(19, ConstantKind::Module as u8);
    assert_eq!(20, ConstantKind::Package as u8);
}

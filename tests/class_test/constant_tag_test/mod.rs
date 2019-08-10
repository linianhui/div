extern crate div;
use div::class::ConstantTag;

#[test]
fn test_constant_tag() {
    assert_eq!(1, ConstantTag::Utf8 as u8);
    assert_eq!(3, ConstantTag::Integer as u8);
    assert_eq!(4, ConstantTag::Float as u8);
    assert_eq!(5, ConstantTag::Long as u8);
    assert_eq!(6, ConstantTag::Double as u8);
    assert_eq!(7, ConstantTag::Class as u8);
    assert_eq!(8, ConstantTag::String as u8);
    assert_eq!(9, ConstantTag::FieldRef as u8);
    assert_eq!(10, ConstantTag::MethodRef as u8);
    assert_eq!(11, ConstantTag::InterfaceMethodRef as u8);
    assert_eq!(12, ConstantTag::NameAndType as u8);
    assert_eq!(15, ConstantTag::MethodHandle as u8);
    assert_eq!(16, ConstantTag::MethodType as u8);
    assert_eq!(17, ConstantTag::Dynamic as u8);
    assert_eq!(18, ConstantTag::InvokeDynamic as u8);
    assert_eq!(19, ConstantTag::Module as u8);
    assert_eq!(20, ConstantTag::Package as u8);
}

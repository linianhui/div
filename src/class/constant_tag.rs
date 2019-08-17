// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4-210

use std::mem::transmute;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConstantTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl From<u8> for ConstantTag {
    fn from(value: u8) -> ConstantTag {
        unsafe { transmute::<u8, ConstantTag>(value) }
    }
}

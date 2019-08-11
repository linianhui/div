// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.2

/*
CONSTANT_InterfaceMethodref_info {
    u1 tag;
    u2 class_index;
    u2 name_and_type_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantInterfaceMethodRef {
    pub tag: ConstantTag,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantInterfaceMethodRef {
    pub fn from(bytes: &[u8]) -> ConstantInterfaceMethodRef {
        ConstantInterfaceMethodRef {
            tag: ConstantTag::InterfaceMethodRef,
            class_index: from_be_bytes_to_u16(&bytes[0..2]),
            name_and_type_index: from_be_bytes_to_u16(&bytes[2..4]),
        }
    }
}
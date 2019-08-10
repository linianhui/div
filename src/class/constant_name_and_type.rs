// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.6

/*
CONSTANT_NameAndType_info {
    u1 tag;
    u2 name_index;
    u2 descriptor_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantNameAndType {
    pub tag: ConstantTag,
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl ConstantNameAndType {
    pub fn from(bytes: &[u8]) -> ConstantNameAndType {
        ConstantNameAndType {
            tag: ConstantTag::NameAndType,
            name_index: from_be_bytes_to_u16(&bytes[0..2]),
            descriptor_index: from_be_bytes_to_u16(&bytes[2..4]),
        }
    }
}

// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.9

/*
CONSTANT_MethodType_info {
    u1 tag;
    u2 descriptor_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantMethodType {
    pub tag: ConstantTag,
    pub descriptor_index: u16,
}

impl ConstantMethodType {
    pub fn from(bytes: &[u8]) -> ConstantMethodType {
        ConstantMethodType {
            tag: ConstantTag::MethodType,
            descriptor_index: from_be_bytes_to_u16(&bytes[0..2]),
        }
    }
}

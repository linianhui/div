// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.3

/*
CONSTANT_String_info {
    u1 tag;
    u2 string_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantString {
    pub tag: ConstantTag,
    pub string_index: u16,
}

impl ConstantString {
    pub fn from(bytes: &[u8]) -> ConstantString {
        ConstantString {
            tag: ConstantTag::String,
            string_index: from_be_bytes_to_u16(&bytes[0..2]),
        }
    }
}

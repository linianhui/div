// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.4

/*
CONSTANT_Integer_info {
    u1 tag;
    u4 bytes;
}
*/

use crate::class::from_be_bytes_to_i32;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantInteger {
    pub tag: ConstantTag,
    pub value: i32,
}

impl ConstantInteger {
    pub fn from(bytes: &[u8]) -> ConstantInteger {
        ConstantInteger {
            tag: ConstantTag::Integer,
            value: from_be_bytes_to_i32(&bytes[0..4]),
        }
    }
}

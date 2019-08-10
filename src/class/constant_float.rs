// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.4

/*
CONSTANT_Float_info {
    u1 tag;
    u4 bytes;
}
*/

use crate::class::from_be_bytes_to_f32;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantFloat {
    pub tag: ConstantTag,
    pub value: f32,
}

impl ConstantFloat {
    pub fn from(bytes: &[u8]) -> ConstantFloat {
        ConstantFloat {
            tag: ConstantTag::Float,
            value: from_be_bytes_to_f32(&bytes[0..4]),
        }
    }
}

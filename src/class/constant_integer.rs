// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.4

/*
CONSTANT_Integer_info {
    u1 tag;
    u4 bytes;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantInteger {
    pub tag: ConstantTag,
    pub bytes: i32,
}

impl ConstantInteger {
    pub fn new(reader: &mut U8Reader) -> ConstantInteger {
        ConstantInteger {
            tag: ConstantTag::Integer,
            bytes: reader.read_i32_and_update_position(),
        }
    }
}

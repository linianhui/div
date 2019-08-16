// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.5

/*
CONSTANT_Long_info {
    u1 tag;
    u4 high_bytes;
    u4 low_bytes;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantLong {
    pub tag: ConstantTag,
    pub value: i64,
}

impl ConstantLong {
    pub fn from(reader: &mut U8Reader) -> ConstantLong {
        ConstantLong {
            tag: ConstantTag::Long,
            value: reader.read_i64(),
        }
    }
}

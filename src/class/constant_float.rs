// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.4

/*
CONSTANT_Float_info {
    u1 tag;
    u4 bytes;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantFloat {
    pub tag: ConstantTag,
    pub value: f32,
}

impl ConstantFloat {
    pub fn new(reader: &mut U8Reader) -> ConstantFloat {
        ConstantFloat {
            tag: ConstantTag::Float,
            value: reader.read_f32(),
        }
    }
}

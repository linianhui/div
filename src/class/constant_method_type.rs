// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.9

/*
CONSTANT_MethodType_info {
    u1 tag;
    u2 descriptor_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantMethodType {
    pub tag: ConstantTag,
    pub descriptor_index: u16,
}

impl ConstantMethodType {
    pub fn new(reader: &mut U8Reader) -> ConstantMethodType {
        ConstantMethodType {
            tag: ConstantTag::MethodType,
            descriptor_index: reader.read_u16_and_update_position(),
        }
    }
}

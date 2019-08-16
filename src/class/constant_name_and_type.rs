// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.6

/*
CONSTANT_NameAndType_info {
    u1 tag;
    u2 name_index;
    u2 descriptor_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantNameAndType {
    pub tag: ConstantTag,
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl ConstantNameAndType {
    pub fn from(reader: &mut U8Reader) -> ConstantNameAndType {
        ConstantNameAndType {
            tag: ConstantTag::NameAndType,
            name_index: reader.read_u16(),
            descriptor_index: reader.read_u16(),
        }
    }
}

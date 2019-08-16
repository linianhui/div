// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.1

/*
CONSTANT_Class_info {
    u1 tag;
    u2 name_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantClass {
    pub tag: ConstantTag,
    pub name_index: u16,
}

impl ConstantClass {
    pub fn from(reader: &mut U8Reader) -> ConstantClass {
        ConstantClass {
            tag: ConstantTag::Class,
            name_index: reader.read_u16(),
        }
    }
}

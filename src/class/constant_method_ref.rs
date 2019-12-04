// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.2

/*
CONSTANT_Methodref_info {
    u1 tag;
    u2 class_index;
    u2 name_and_type_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantMethodRef {
    pub tag: ConstantTag,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantMethodRef {
    pub fn new(reader: &mut U8Reader) -> ConstantMethodRef {
        ConstantMethodRef {
            tag: ConstantTag::MethodRef,
            class_index: reader.read_u16(),
            name_and_type_index: reader.read_u16(),
        }
    }
}

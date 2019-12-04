// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.11

/*
CONSTANT_Module_info {
    u1 tag;
    u2 name_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantModule {
    pub tag: ConstantTag,
    pub name_index: u16,
}

impl ConstantModule {
    pub fn new(reader: &mut U8Reader) -> ConstantModule {
        ConstantModule {
            tag: ConstantTag::Module,
            name_index: reader.read_u16(),
        }
    }
}

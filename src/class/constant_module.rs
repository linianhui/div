// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.11

/*
CONSTANT_Module_info {
    u1 tag;
    u2 name_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantModule {
    pub tag: ConstantTag,
    pub name_index: u16,
}

impl ConstantModule {
    pub fn from(bytes: &[u8]) -> ConstantModule {
        ConstantModule {
            tag: ConstantTag::Module,
            name_index: from_be_bytes_to_u16(&bytes[0..2]),
        }
    }
}

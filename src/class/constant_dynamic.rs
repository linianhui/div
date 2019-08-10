// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.10

/*
CONSTANT_Dynamic_info {
    u1 tag;
    u2 bootstrap_method_attr_index;
    u2 name_and_type_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantDynamic {
    pub tag: ConstantTag,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantDynamic {
    pub fn from(bytes: &[u8]) -> ConstantDynamic {
        ConstantDynamic {
            tag: ConstantTag::Dynamic,
            bootstrap_method_attr_index: from_be_bytes_to_u16(&bytes[0..2]),
            name_and_type_index: from_be_bytes_to_u16(&bytes[2..4]),
        }
    }
}

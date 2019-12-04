// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.10

/*
CONSTANT_Dynamic_info {
    u1 tag;
    u2 bootstrap_method_attr_index;
    u2 name_and_type_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantDynamic {
    pub tag: ConstantTag,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantDynamic {
    pub fn new(reader: &mut U8Reader) -> ConstantDynamic {
        ConstantDynamic {
            tag: ConstantTag::Dynamic,
            bootstrap_method_attr_index: reader.read_u16_and_update_position(),
            name_and_type_index: reader.read_u16_and_update_position(),
        }
    }
}

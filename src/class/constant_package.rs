// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.12

/*
CONSTANT_Package_info {
    u1 tag;
    u2 name_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantPackage {
    pub tag: ConstantTag,
    pub name_index: u16,
}

impl ConstantPackage {
    pub fn from(reader: &mut U8Reader) -> ConstantPackage {
        ConstantPackage {
            tag: ConstantTag::Package,
            name_index: reader.read_u16(),
        }
    }
}

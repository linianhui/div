// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.12

/*
CONSTANT_Package_info {
    u1 tag;
    u2 name_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;

#[derive(Debug)]
pub struct ConstantPackage {
    pub tag: ConstantTag,
    pub name_index: u16,
}

impl ConstantPackage {
    pub fn from(bytes: &[u8]) -> ConstantPackage {
        ConstantPackage {
            tag: ConstantTag::Package,
            name_index: from_be_bytes_to_u16(&bytes[0..2]),
        }
    }
}

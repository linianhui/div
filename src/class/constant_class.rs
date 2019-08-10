// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.1

/*
CONSTANT_Class_info {
    u1 tag;
    u2 name_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantKind;

#[derive(Debug)]
pub struct ConstantClass {
    pub kind: ConstantKind,
    pub name_index: u16,
}

impl ConstantClass {
    pub fn from(bytes: &[u8]) -> ConstantClass {
        ConstantClass {
            kind: ConstantKind::Class,
            name_index: from_be_bytes_to_u16(&bytes[0..2]),
        }
    }
}

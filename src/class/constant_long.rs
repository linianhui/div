// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.5

/*
CONSTANT_Long_info {
    u1 tag;
    u4 high_bytes;
    u4 low_bytes;
}
*/

use crate::class::from_be_bytes_to_i64;
use crate::class::ConstantKind;

#[derive(Debug)]
pub struct ConstantLong {
    pub kind: ConstantKind,
    pub value: i64,
}

impl ConstantLong {
    pub fn from(bytes: &[u8]) -> ConstantLong {
        ConstantLong {
            kind: ConstantKind::Long,
            value: from_be_bytes_to_i64(&bytes[0..8]),
        }
    }
}

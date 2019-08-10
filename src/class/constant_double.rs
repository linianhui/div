// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.5

/*
CONSTANT_Double_info {
    u1 tag;
    u4 high_bytes;
    u4 low_bytes;
}
*/

use crate::class::from_be_bytes_to_f64;
use crate::class::ConstantKind;

#[derive(Debug)]
pub struct ConstantDouble {
    pub kind: ConstantKind,
    pub value: f64,
}

impl ConstantDouble {
    pub fn from(bytes: &[u8]) -> ConstantDouble {
        ConstantDouble {
            kind: ConstantKind::Double,
            value: from_be_bytes_to_f64(&bytes[0..8]),
        }
    }
}

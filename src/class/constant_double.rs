// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.5

/*
CONSTANT_Double_info {
    u1 tag;
    u4 high_bytes;
    u4 low_bytes;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantDouble {
    pub tag: ConstantTag,
    pub value: f64,
}

impl ConstantDouble {
    pub fn new(reader: &mut U8Reader) -> ConstantDouble {
        ConstantDouble {
            tag: ConstantTag::Double,
            value: reader.read_f64_and_update_position(),
        }
    }
}

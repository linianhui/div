// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.3

/*
CONSTANT_String_info {
    u1 tag;
    u2 string_index;
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantString {
    pub tag: ConstantTag,
    pub string_index: u16,
}

impl ConstantString {
    pub fn new(reader: &mut U8Reader) -> ConstantString {
        ConstantString {
            tag: ConstantTag::String,
            string_index: reader.read_u16_and_update_position(),
        }
    }
}

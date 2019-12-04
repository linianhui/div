// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.7

/*
CONSTANT_Utf8_info {
    u1 tag;
    u2 length;
    u1 bytes[length];
}
*/

use crate::class::ConstantTag;
use crate::class::U8Reader;
use std::convert::TryInto;

#[derive(Debug)]
pub struct ConstantUtf8 {
    pub tag: ConstantTag,
    pub length: u16,
    pub bytes: Box<[u8]>,
}

impl ConstantUtf8 {
    pub fn new(reader: &mut U8Reader) -> ConstantUtf8 {
        let length = reader.read_u16_and_update_position();
        let bytes = reader.read_bytes_and_update_position(length.try_into().unwrap());
        ConstantUtf8 {
            tag: ConstantTag::Utf8,
            length: length,
            bytes: Box::from(bytes),
        }
    }
}

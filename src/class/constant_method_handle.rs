// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.8

/*
CONSTANT_MethodHandle_info {
    u1 tag;
    u1 reference_kind;
    u2 reference_index;
}
*/

use crate::class::ConstantTag;
use crate::class::MethodHandleReferenceKind;
use crate::class::U8Reader;

#[derive(Debug)]
pub struct ConstantMethodHandle {
    pub tag: ConstantTag,
    pub reference_kind: MethodHandleReferenceKind,
    pub reference_index: u16,
}

impl ConstantMethodHandle {
    pub fn new(reader: &mut U8Reader) -> ConstantMethodHandle {
        ConstantMethodHandle {
            tag: ConstantTag::MethodHandle,
            reference_kind: MethodHandleReferenceKind::from(reader.read_u8_and_update_position()),
            reference_index: reader.read_u16_and_update_position(),
        }
    }
}

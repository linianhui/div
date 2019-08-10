// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.4.8

/*
CONSTANT_MethodHandle_info {
    u1 tag;
    u1 reference_kind;
    u2 reference_index;
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::ConstantTag;
use crate::class::MethodHandleReferenceKind;
use std::mem;

#[derive(Debug)]
pub struct ConstantMethodHandle {
    pub tag: ConstantTag,
    pub reference_kind: MethodHandleReferenceKind,
    pub reference_index: u16,
}

impl ConstantMethodHandle {
    pub fn from(bytes: &[u8]) -> ConstantMethodHandle {
        unsafe {
            ConstantMethodHandle {
                tag: ConstantTag::MethodHandle,
                reference_kind: mem::transmute::<u8, MethodHandleReferenceKind>(bytes[0]),
                reference_index: from_be_bytes_to_u16(&bytes[1..3]),
            }
        }
    }
}

// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.5-200-A.1

use std::mem::transmute;

#[repr(u16)]
#[derive(Debug, PartialEq)]
pub enum FieldAccessFlags {
    Public = 0x_00_01,
    Private = 0x_00_02,
    Protected = 0x_00_04,
    Static = 0x_00_08,
    Final = 0x_00_10,
    Volatile = 0x_00_40,
    Transient = 0x_00_80,
    Synthetic = 0x_10_00,
    Enum = 0x_40_00,
}

impl From<u16> for FieldAccessFlags {
    fn from(value: u16) -> FieldAccessFlags {
        unsafe { transmute::<u16, FieldAccessFlags>(value) }
    }
}

// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.5-200-A.1

use std::collections::HashSet;
use std::mem::transmute;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Hash)]
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

impl FieldAccessFlags {
    pub fn flags(value: u16) -> HashSet<FieldAccessFlags> {
        let mut all: HashSet<FieldAccessFlags> = HashSet::new();

        if value & 0x_00_01 != 0 {
            all.insert(FieldAccessFlags::Public);
        }

        if value & 0x_00_02 != 0 {
            all.insert(FieldAccessFlags::Private);
        }

        if value & 0x_00_04 != 0 {
            all.insert(FieldAccessFlags::Protected);
        }

        if value & 0x_00_08 != 0 {
            all.insert(FieldAccessFlags::Static);
        }

        if value & 0x_00_10 != 0 {
            all.insert(FieldAccessFlags::Final);
        }

        if value & 0x_00_40 != 0 {
            all.insert(FieldAccessFlags::Volatile);
        }

        if value & 0x_00_80 != 0 {
            all.insert(FieldAccessFlags::Transient);
        }

        if value & 0x_10_00 != 0 {
            all.insert(FieldAccessFlags::Synthetic);
        }

        if value & 0x_40_00 != 0 {
            all.insert(FieldAccessFlags::Enum);
        }

        return all;
    }
}

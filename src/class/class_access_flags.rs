// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.1-200-E.1

use std::collections::HashSet;
use std::mem::transmute;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ClassAccessFlags {
    Public = 0x_00_01,
    Final = 0x_00_10,
    Super = 0x_00_20,
    Interface = 0x_02_00,
    Abstract = 0x_04_00,
    Synthetic = 0x_10_00,
    Annotation = 0x_20_00,
    Enum = 0x_40_00,
    Module = 0x_80_00,
}

impl From<u16> for ClassAccessFlags {
    fn from(value: u16) -> ClassAccessFlags {
        unsafe { transmute::<u16, ClassAccessFlags>(value) }
    }
}

impl ClassAccessFlags {
    pub fn flags(value: u16) -> HashSet<ClassAccessFlags> {
        let mut all: HashSet<ClassAccessFlags> = HashSet::new();

        if value & 0x_00_01 != 0 {
            all.insert(ClassAccessFlags::Public);
        }

        if value & 0x_00_10 != 0 {
            all.insert(ClassAccessFlags::Final);
        }

        if value & 0x_00_20 != 0 {
            all.insert(ClassAccessFlags::Super);
        }

        if value & 0x_02_00 != 0 {
            all.insert(ClassAccessFlags::Interface);
        }

        if value & 0x_04_00 != 0 {
            all.insert(ClassAccessFlags::Abstract);
        }

        if value & 0x_10_00 != 0 {
            all.insert(ClassAccessFlags::Synthetic);
        }

        if value & 0x_20_00 != 0 {
            all.insert(ClassAccessFlags::Annotation);
        }

        if value & 0x_40_00 != 0 {
            all.insert(ClassAccessFlags::Enum);
        }

        if value & 0x_80_00 != 0 {
            all.insert(ClassAccessFlags::Module);
        }

        return all;
    }
}

// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.6-200-A.1

use std::collections::HashSet;
use std::mem::transmute;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MethodAccessFlags {
    Public = 0x_00_01,
    Private = 0x_00_02,
    Protected = 0x_00_04,
    Static = 0x_00_08,
    Final = 0x_00_10,
    Synchronized = 0x_00_20,
    Bridge = 0x_00_40,
    Varargs = 0x_00_80,
    Native = 0x_01_00,
    Abstract = 0x_04_00,
    Strict = 0x_08_00,
    Synthetic = 0x_10_00,
}

impl From<u16> for MethodAccessFlags {
    fn from(value: u16) -> MethodAccessFlags {
        unsafe { transmute::<u16, MethodAccessFlags>(value) }
    }
}

impl MethodAccessFlags {
    pub fn flags(value: u16) -> HashSet<MethodAccessFlags> {
        println!("{}", value);
        let mut all: HashSet<MethodAccessFlags> = HashSet::new();

        if value & 0x_00_01 != 0 {
            all.insert(MethodAccessFlags::Public);
        }

        if value & 0x_00_02 != 0 {
            all.insert(MethodAccessFlags::Private);
        }

        if value & 0x_00_04 != 0 {
            all.insert(MethodAccessFlags::Protected);
        }

        if value & 0x_00_08 != 0 {
            all.insert(MethodAccessFlags::Static);
        }

        if value & 0x_00_10 != 0 {
            all.insert(MethodAccessFlags::Final);
        }

        if value & 0x_00_20 != 0 {
            all.insert(MethodAccessFlags::Synchronized);
        }

        if value & 0x_00_40 != 0 {
            all.insert(MethodAccessFlags::Bridge);
        }

        if value & 0x_00_80 != 0 {
            all.insert(MethodAccessFlags::Varargs);
        }

        if value & 0x_01_00 != 0 {
            all.insert(MethodAccessFlags::Native);
        }

        if value & 0x_04_00 != 0 {
            all.insert(MethodAccessFlags::Abstract);
        }

        if value & 0x_08_00 != 0 {
            all.insert(MethodAccessFlags::Strict);
        }

        if value & 0x_10_00 != 0 {
            all.insert(MethodAccessFlags::Synthetic);
        }

        return all;
    }
}

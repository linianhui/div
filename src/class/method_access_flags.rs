// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.6-200-A.1

#[repr(u16)]
#[derive(Debug, PartialEq)]
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

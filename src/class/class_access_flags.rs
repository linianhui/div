// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.1-200-E.1

#[repr(u16)]
#[derive(Debug, PartialEq)]
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

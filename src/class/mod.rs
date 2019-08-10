mod class_file;
mod constant_integer;
mod constant_kind;
pub use class_file::ClassFile;
pub use constant_integer::ConstantInteger;
pub use constant_kind::ConstantKind;

pub fn from_be_bytes_to_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

pub fn from_be_bytes_to_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

pub fn from_be_bytes_to_i32(bytes: &[u8]) -> i32 {
    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

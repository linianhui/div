mod class_file;
mod constant_class;
mod constant_double;
mod constant_field_ref;
mod constant_float;
mod constant_integer;
mod constant_kind;
mod constant_long;
mod constant_method_ref;
pub use class_file::ClassFile;
pub use constant_class::ConstantClass;
pub use constant_double::ConstantDouble;
pub use constant_field_ref::ConstantFieldRef;
pub use constant_float::ConstantFloat;
pub use constant_integer::ConstantInteger;
pub use constant_kind::ConstantKind;
pub use constant_long::ConstantLong;
pub use constant_method_ref::ConstantMethodRef;

pub fn from_be_bytes_to_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

pub fn from_be_bytes_to_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

pub fn from_be_bytes_to_u64(bytes: &[u8]) -> u64 {
    u64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

pub fn from_be_bytes_to_i32(bytes: &[u8]) -> i32 {
    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

pub fn from_be_bytes_to_i64(bytes: &[u8]) -> i64 {
    i64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

pub fn from_be_bytes_to_f32(bytes: &[u8]) -> f32 {
    f32::from_bits(from_be_bytes_to_u32(bytes))
}

pub fn from_be_bytes_to_f64(bytes: &[u8]) -> f64 {
    f64::from_bits(from_be_bytes_to_u64(bytes))
}

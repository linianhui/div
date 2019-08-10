// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html
/*
ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/

use crate::class::from_be_bytes_to_u16;
use crate::class::from_be_bytes_to_u32;

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
}

impl ClassFile {
    pub fn from(bytes: &[u8]) -> ClassFile {
        ClassFile {
            magic: from_be_bytes_to_u32(&bytes[0..4]),
            minor_version: from_be_bytes_to_u16(&bytes[4..6]),
            major_version: from_be_bytes_to_u16(&bytes[6..8]),
            constant_pool_count: from_be_bytes_to_u16(&bytes[8..10]),
        }
    }
}

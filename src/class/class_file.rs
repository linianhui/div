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

use crate::class::U8Reader;

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
}

impl ClassFile {
    pub fn new(bytes: &[u8]) -> ClassFile {
        let mut reader = U8Reader::new(bytes);
        let magic = reader.read_u32();
        let minor_version = reader.read_u16();
        let major_version = reader.read_u16();
        let constant_pool_count = reader.read_u16();
        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
        }
    }
}

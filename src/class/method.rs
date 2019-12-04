// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.6

/*
method_info {
    u2             access_flags;
    u2             name_index;
    u2             descriptor_index;
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/

use crate::class::Attribute;
use crate::class::MethodAccessFlags;
use crate::class::U8Reader;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Method {
    pub access_flags: HashSet<MethodAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn new(reader: &mut U8Reader) -> Method {
        let access_flags = MethodAccessFlags::flags(reader.read_u16_and_update_position());
        let name_index = reader.read_u16_and_update_position();
        let descriptor_index = reader.read_u16_and_update_position();
        let attributes_count = reader.read_u16_and_update_position();
        let attributes = Attribute::vec(attributes_count as usize, reader);

        Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }

    pub fn vec(method_count: usize, reader: &mut U8Reader) -> Vec<Method> {
        let mut methods: Vec<Method> = Vec::with_capacity(method_count);
        for _i in 0..method_count {
            let method = Method::new(reader);
            methods.push(method);
        }
        return methods;
    }
}

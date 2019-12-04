// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.5

/*
field_info {
    u2             access_flags;
    u2             name_index;
    u2             descriptor_index;
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/

use crate::class::Attribute;
use crate::class::FieldAccessFlags;
use crate::class::U8Reader;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Field {
    pub access_flags: HashSet<FieldAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

impl Field {
    pub fn new(reader: &mut U8Reader) -> Field {
        let access_flags = FieldAccessFlags::flags(reader.read_u16());
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        let attributes_count = reader.read_u16();
        let attributes = Attribute::vec(attributes_count as usize, reader);

        Field {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }

    pub fn vec(field_count: usize, reader: &mut U8Reader) -> Vec<Field> {
        let mut fields: Vec<Field> = Vec::with_capacity(field_count);
        for _i in 0..field_count {
            let field = Field::new(reader);
            fields.push(field);
        }
        return fields;
    }
}

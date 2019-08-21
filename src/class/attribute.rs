// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html#jvms-4.7

/*
attribute_info {
    u2 attribute_name_index;
    u4 attribute_length;
    u1 info[attribute_length];
}
*/

use crate::class::U8Reader;

#[derive(Debug)]
pub struct Attribute {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub bytes: Box<[u8]>,
}

impl Attribute {
    pub fn new(reader: &mut U8Reader) -> Attribute {
        let attribute_name_index = reader.read_u16();
        let attribute_length = reader.read_u32();
        let bytes = reader.read_bytes(attribute_length as usize);

        Attribute {
            attribute_name_index,
            attribute_length,
            bytes: Box::from(bytes),
        }
    }

    pub fn vec(attribute_count: usize, reader: &mut U8Reader) -> Vec<Attribute> {
        let mut attributes: Vec<Attribute> = Vec::with_capacity(attribute_count);
        for _i in 0..attribute_count {
            let attribute = Attribute::new(reader);
            attributes.push(attribute);
        }
        return attributes;
    }
}

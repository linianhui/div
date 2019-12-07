extern crate div;
use div::class::Attribute;
use div::class::U8Reader;

#[test]
fn test_attribute_vec() {
    let mut u8_reader = U8Reader::new(&[
        0x00, 0x38, 0x00, 0x00, 0x00, 0x02, 0x00, 0x39, 0x00, 0x3A, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x3B,
    ]);
    let attributes = Attribute::vec(&mut u8_reader, 2);

    assert_eq!(2, attributes.len());
    assert_eq!(56, attributes[0].attribute_name_index);
    assert_eq!(2, attributes[0].attribute_length);
    assert_eq!(58, attributes[1].attribute_name_index);
    assert_eq!(2, attributes[1].attribute_length);
}

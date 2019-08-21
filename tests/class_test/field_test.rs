extern crate div;
use div::class::Field;
use div::class::FieldAccessFlags;
use div::class::U8Reader;

#[test]
fn test_field_vec() {
    let mut u8_reader = U8Reader::new(&[
        0x00, 0x19, 0x00, 0x16, 0x00, 0x17, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x19, 0x00, 0x19, 0x00, 0x1A, 0x00, 0x1B, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x1C,
    ]);
    let fields = Field::vec(2, &mut u8_reader);

    assert_eq!(2, fields.len());

    assert_eq!(3, fields[0].access_flags.len());
    assert!(fields[0].access_flags.contains(&FieldAccessFlags::Public));
    assert!(fields[0].access_flags.contains(&FieldAccessFlags::Static));
    assert!(fields[0].access_flags.contains(&FieldAccessFlags::Final));
    assert_eq!(22, fields[0].name_index);
    assert_eq!(23, fields[0].descriptor_index);
    assert_eq!(1, fields[0].attributes_count);
    assert_eq!(1, fields[0].attributes.len());
    assert_eq!(24, fields[0].attributes[0].attribute_name_index);
    assert_eq!(2, fields[0].attributes[0].attribute_length);

    assert_eq!(3, fields[1].access_flags.len());
    assert!(fields[1].access_flags.contains(&FieldAccessFlags::Public));
    assert!(fields[1].access_flags.contains(&FieldAccessFlags::Static));
    assert!(fields[1].access_flags.contains(&FieldAccessFlags::Final));
    assert_eq!(26, fields[1].name_index);
    assert_eq!(27, fields[1].descriptor_index);
    assert_eq!(1, fields[1].attributes_count);
    assert_eq!(1, fields[1].attributes.len());
    assert_eq!(24, fields[1].attributes[0].attribute_name_index);
    assert_eq!(2, fields[1].attributes[0].attribute_length);
}

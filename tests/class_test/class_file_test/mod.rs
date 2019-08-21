extern crate div;
use div::class::*;

#[test]
fn test_class_file() {
    let class_file = ClassFile::new(include_bytes!("class_file_test.class"));

    assert_eq!(0xCA_FE_BA_BE, class_file.magic);

    assert_eq!(0x00_00, class_file.minor_version);

    assert_eq!(0x00_34, class_file.major_version);

    assert_eq!(90, class_file.constant_pool_count);

    assert_eq!(90, class_file.constant_pool.len());

    assert_eq!(2, class_file.access_flags.len());
    assert!(class_file.access_flags.contains(&ClassAccessFlags::Public));
    assert!(class_file.access_flags.contains(&ClassAccessFlags::Super));

    assert_eq!(5, class_file.this_class);

    assert_eq!(20, class_file.super_class);

    assert_eq!(1, class_file.interfaces_count);

    assert_eq!(vec![21], class_file.interfaces);

    assert_eq!(9, class_file.fields_count);

    let fields = class_file.fields;
    assert_eq!(9, fields.len());
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

    assert_eq!(4, class_file.methods_count);

    let methods = class_file.methods;
    assert_eq!(4, methods.len());
    assert_eq!(1, methods[0].access_flags.len());
    assert!(methods[0].access_flags.contains(&MethodAccessFlags::Public));
    assert_eq!(45, methods[0].name_index);
    assert_eq!(46, methods[0].descriptor_index);
    assert_eq!(1, methods[0].attributes_count);
    assert_eq!(1, methods[0].attributes.len());
    assert_eq!(47, methods[0].attributes[0].attribute_name_index);
    assert_eq!(29, methods[0].attributes[0].attribute_length);
    assert_eq!(2, methods[1].access_flags.len());
    assert!(methods[1].access_flags.contains(&MethodAccessFlags::Public));
    assert!(methods[1].access_flags.contains(&MethodAccessFlags::Static));
    assert_eq!(49, methods[1].name_index);
    assert_eq!(50, methods[1].descriptor_index);
    assert_eq!(2, methods[1].attributes_count);
    assert_eq!(2, methods[1].attributes.len());
    assert_eq!(47, methods[1].attributes[0].attribute_name_index);
    assert_eq!(147, methods[1].attributes[0].attribute_length);

    assert_eq!(2, class_file.attributes_count);

    let attributes = class_file.attributes;

    assert_eq!(2, attributes.len());
    assert_eq!(56, attributes[0].attribute_name_index);
    assert_eq!(2, attributes[0].attribute_length);
    assert_eq!(58, attributes[1].attribute_name_index);
    assert_eq!(2, attributes[1].attribute_length);
}

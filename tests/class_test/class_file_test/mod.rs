extern crate div;
use div::class::*;

#[test]
fn test_class_file_magic() {
    let class_file = read_class_file();
    assert_eq!(0xCA_FE_BA_BE, class_file.magic);
}

#[test]
fn test_class_file_minor_version() {
    let class_file = read_class_file();
    assert_eq!(0x00_00, class_file.minor_version);
}

#[test]
fn test_class_file_major_version() {
    let class_file = read_class_file();
    assert_eq!(0x00_34, class_file.major_version);
}

#[test]
fn test_class_file_constant_pool_count() {
    let class_file = read_class_file();
    assert_eq!(81, class_file.constant_pool_count);
}

#[test]
fn test_class_file_constant_pool() {
    let class_file = read_class_file();
    let constant_pool = class_file.constant_pool;
    assert_eq!(81, constant_pool.len());
}

#[test]
fn test_class_file_access_flags() {
    let class_file = read_class_file();
    let access_flags = class_file.access_flags;
    assert_eq!(2, access_flags.len());
    assert_eq!(true, access_flags.contains(&ClassAccessFlags::Public));
    assert_eq!(true, access_flags.contains(&ClassAccessFlags::Super));
}

#[test]
fn test_class_file_this_class() {
    let class_file = read_class_file();
    assert_eq!(5, class_file.this_class);
}

#[test]
fn test_class_file_super_class() {
    let class_file = read_class_file();
    assert_eq!(19, class_file.super_class);
}

#[test]
fn test_class_file_interfaces_count() {
    let class_file = read_class_file();
    assert_eq!(0, class_file.interfaces_count);
}

fn read_class_file() -> ClassFile {
    ClassFile::new(include_bytes!("class_file_test.class"))
}

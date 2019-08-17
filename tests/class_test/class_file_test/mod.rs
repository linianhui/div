extern crate div;
use div::class::ClassFile;

#[test]
fn test_class_file() {
    let class_file = read_class_file();
    assert_eq!(0xCA_FE_BA_BE, class_file.magic);
    assert_eq!(0x00_00, class_file.minor_version);
    assert_eq!(0x00_34, class_file.major_version);
    assert_eq!(29, class_file.constant_pool_count);
    assert_eq!(29, class_file.constant_pool.len());
}

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
    assert_eq!(29, class_file.constant_pool_count);
}

fn read_class_file() -> ClassFile {
    ClassFile::new(include_bytes!("test.class"))
}

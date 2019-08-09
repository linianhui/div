extern crate div;
use div::class::ClassFile;

#[test]
fn test_class_file() {
    let class_file = read_class_file();
    assert_eq!(0xCA_FE_BA_BE, class_file.magic);
    assert_eq!(0x00_00, class_file.minor_version);
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

fn read_class_file() -> ClassFile {
    ClassFile::from(include_bytes!("test.class"))
}

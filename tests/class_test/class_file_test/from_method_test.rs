extern crate div;
use div::class::ClassFile;

#[test]
fn test_class_file() {
    let class_file_bytes = include_bytes!("test.class");
    let class_file = ClassFile::from(class_file_bytes);
    assert_eq!(0xCA_FE_BA_BE, class_file.magic);
}

#[test]
fn test_class_file_magic() {
    let class_file_bytes = [0xCA, 0xFE, 0xBA, 0xBE];
    let class_file = ClassFile::from(&class_file_bytes);
    assert_eq!(0xCA_FE_BA_BE, class_file.magic);
}

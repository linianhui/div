extern crate jvm;

#[test]
fn test_from() {
    let class_file = jvm::clazz::ClassFile::from();
    assert_eq!(0xCA_FE_BA_BE, class_file.magic);
    println!("{:#?}", &class_file);
}

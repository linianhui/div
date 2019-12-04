extern crate div;
use div::class::ConstantPackage;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_package() {
    let mut u8_reader = U8Reader::new(&[0x00, 0x15]);
    let constant_package = ConstantPackage::new(&mut u8_reader);
    assert_eq!(ConstantTag::Package, constant_package.tag);
    assert_eq!(21, constant_package.name_index);
    assert_eq!(2, u8_reader.position);
}

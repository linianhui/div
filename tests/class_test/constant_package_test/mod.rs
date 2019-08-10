extern crate div;
use div::class::ConstantPackage;
use div::class::ConstantTag;

#[test]
fn test_constant_package() {
    let constant_package = ConstantPackage::from(&[0x00, 0x15]);
    assert_eq!(ConstantTag::Package, constant_package.tag);
    assert_eq!(21, constant_package.name_index);
}

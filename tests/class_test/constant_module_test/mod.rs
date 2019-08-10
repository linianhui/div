extern crate div;
use div::class::ConstantModule;
use div::class::ConstantTag;

#[test]
fn test_constant_module() {
    let constant_module = ConstantModule::from(&[0x00, 0x15]);
    assert_eq!(ConstantTag::Module, constant_module.tag);
    assert_eq!(21, constant_module.name_index);
}

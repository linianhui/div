extern crate div;
use div::class::ConstantModule;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_module() {
    let mut u8_reader = U8Reader::from(&[0x00, 0x15]);
    let constant_module = ConstantModule::from(&mut u8_reader);
    assert_eq!(ConstantTag::Module, constant_module.tag);
    assert_eq!(21, constant_module.name_index);
}

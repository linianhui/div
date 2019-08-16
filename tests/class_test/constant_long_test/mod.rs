extern crate div;
use div::class::ConstantLong;
use div::class::ConstantTag;
use div::class::U8Reader;

#[test]
fn test_constant_long() {
    let mut u8_reader = U8Reader::from(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
    let constant_long = ConstantLong::from(&mut u8_reader);
    assert_eq!(ConstantTag::Long, constant_long.tag);
    assert_eq!(0x12_34_56_78_9A_BC_DE_F0, constant_long.value);
}

extern crate div;
use div::class::ConstantKind;
use div::class::ConstantLong;

#[test]
fn test_constant_long() {
    let constant_long = ConstantLong::from(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
    assert_eq!(ConstantKind::Long, constant_long.kind);
    assert_eq!(0x12_34_56_78_9A_BC_DE_F0, constant_long.value);
}

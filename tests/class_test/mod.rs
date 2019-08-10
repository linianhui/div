mod class_file_test;
mod constant_integer_test;
mod constant_kind_test;

extern crate div;
use div::class;

#[test]
fn test_from_be_bytes_to_u16() {
    let bytes = [0xCA, 0xFE];
    let u16_value = class::from_be_bytes_to_u16(&bytes);
    assert_eq!(0xCA_FE, u16_value);
}

#[test]
fn test_from_be_bytes_to_u32() {
    let bytes = [0xCA, 0xFE, 0xBA, 0xBE];
    let u32_value = class::from_be_bytes_to_u32(&bytes);
    assert_eq!(0xCA_FE_BA_BE, u32_value);
}

#[test]
fn test_from_be_bytes_to_i32() {
    let bytes = [0x12, 0x34, 0x56, 0x78];
    let i32_value = class::from_be_bytes_to_i32(&bytes);
    assert_eq!(0x12_34_56_78, i32_value);
}

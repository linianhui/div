mod from_method_test;

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

mod class_access_flags_test;
mod class_file_test;
mod constant_class_test;
mod constant_double_test;
mod constant_dynamic_test;
mod constant_field_ref_test;
mod constant_float_test;
mod constant_integer_test;
mod constant_interface_method_ref_test;
mod constant_invoke_dynamic_test;
mod constant_long_test;
mod constant_method_handle_test;
mod constant_method_ref_test;
mod constant_method_type_test;
mod constant_module_test;
mod constant_name_and_type_test;
mod constant_package_test;
mod constant_string_test;
mod constant_tag_test;
mod method_handle_reference_kind_test;

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
fn test_from_be_bytes_to_u64() {
    let bytes = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let u64_value = class::from_be_bytes_to_u64(&bytes);
    assert_eq!(0x12_34_56_78_9A_BC_DE_F0, u64_value);
}

#[test]
fn test_from_be_bytes_to_i32() {
    let bytes = [0x12, 0x34, 0x56, 0x78];
    let i32_value = class::from_be_bytes_to_i32(&bytes);
    assert_eq!(0x12_34_56_78, i32_value);
}

#[test]
fn test_from_be_bytes_to_i64() {
    let bytes = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let i64_value = class::from_be_bytes_to_i64(&bytes);
    assert_eq!(0x12_34_56_78_9A_BC_DE_F0, i64_value);
}

#[test]
fn test_from_be_bytes_to_f32() {
    let bytes = [0x40, 0x49, 0x0F, 0xDB];
    let f32_value = class::from_be_bytes_to_f32(&bytes);
    assert_eq!(3.1415_9265_f32, f32_value);
}

#[test]
fn test_from_be_bytes_to_f64() {
    let bytes = [0x40, 0x09, 0x21, 0xFB, 0x54, 0x43, 0xD1, 0xA8];
    let f64_value = class::from_be_bytes_to_f64(&bytes);
    assert_eq!(3.1415_9265_3579_3978_f64, f64_value);
}

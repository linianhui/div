extern crate div;
use div::class::U8Reader;

#[test]
fn test_read_f32() {
    let mut reader = U8Reader::new(&[0x40, 0x49, 0x0F, 0xDB]);

    let f32_value = reader.read_f32();

    assert_eq!(3.1415_9265_f32, f32_value);
    assert_eq!(4, reader.position);
}

#[test]
fn test_read_f64() {
    let mut reader = U8Reader::new(&[0x40, 0x09, 0x21, 0xFB, 0x54, 0x43, 0xD1, 0xA8]);

    let f64_value = reader.read_f64();

    assert_eq!(3.1415_9265_3579_3978_f64, f64_value);
    assert_eq!(8, reader.position);
}

#[test]
fn test_read_i32() {
    let mut reader = U8Reader::new(&[0x12, 0x34, 0x56, 0x78]);

    let i32_value = reader.read_i32();

    assert_eq!(0x12_34_56_78, i32_value);
    assert_eq!(4, reader.position);
}

#[test]
fn test_read_i64() {
    let mut reader = U8Reader::new(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);

    let i64_value = reader.read_i64();

    assert_eq!(0x12_34_56_78_9A_BC_DE_F0, i64_value);
    assert_eq!(8, reader.position);
}

#[test]
fn test_read_u16() {
    let mut reader = U8Reader::new(&[0xCA, 0xFE, 0xBA, 0xBE]);

    let u16_value_1 = reader.read_u16();
    let u16_value_2 = reader.read_u16();

    assert_eq!(0xCA_FE, u16_value_1);
    assert_eq!(0xBA_BE, u16_value_2);
    assert_eq!(4, reader.position);
}

#[test]
fn test_read_u16_vec() {
    let mut reader = U8Reader::new(&[0xCA, 0xFE, 0xBA, 0xBE]);

    let u16_vec = reader.read_u16_vec(2usize);

    assert_eq!(vec![0xCA_FEu16, 0xBA_BEu16], u16_vec);
    assert_eq!(4, reader.position);
}

#[test]
fn test_read_u32() {
    let mut reader = U8Reader::new(&[0xCA, 0xFE, 0xBA, 0xBE, 0x12, 0x34]);

    let u32_value = reader.read_u32();
    let u16_value = reader.read_u16();

    assert_eq!(0xCA_FE_BA_BE, u32_value);
    assert_eq!(0x12_34, u16_value);
    assert_eq!(6, reader.position);
}

#[test]
fn test_read_u64() {
    let mut reader = U8Reader::new(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);

    let u64_value = reader.read_u64();

    assert_eq!(0x12_34_56_78_9A_BC_DE_F0, u64_value);
    assert_eq!(8, reader.position);
}

#[test]
fn test_read_bytes() {
    let mut reader = U8Reader::new(&[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);

    let bytes_1 = reader.read_bytes(2);
    assert_eq!(&[0x12, 0x34], bytes_1);

    let bytes_2 = reader.read_bytes(3);
    assert_eq!(&[0x56, 0x78, 0x9A], bytes_2);
    assert_eq!(5, reader.position);
}

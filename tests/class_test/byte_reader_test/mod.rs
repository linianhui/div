extern crate div;
use div::class::ByteReader;

#[test]
fn test_read_u16() {
    let mut reader = new_reader(&[0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(0xCA_FE, reader.read_u16());
    assert_eq!(0xBA_BE, reader.read_u16());
}

#[test]
fn test_read_u32() {
    let mut reader = new_reader(&[0xCA, 0xFE, 0xBA, 0xBE, 0x12, 0x34]);
    assert_eq!(0xCA_FE_BA_BE, reader.read_u32());
    assert_eq!(0x12_34, reader.read_u16());
}

fn new_reader(bytes: &[u8]) -> ByteReader {
    ByteReader::from(Box::from(bytes))
}

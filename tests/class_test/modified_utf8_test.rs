extern crate div;
use div::class::ModifiedUtf8;

#[test]
fn test_to_string() {
    let modified_utf8_bytes = &[
        0x4C, 0x4E, 0x48, 0xE4, 0xB8, 0xAD, 0xE6, 0x96, 0x87, 0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80,
    ];

    let string = ModifiedUtf8::to_string(modified_utf8_bytes);

    assert_eq!(15, modified_utf8_bytes.len());
    assert_eq!("LNH中文😀", string);
}

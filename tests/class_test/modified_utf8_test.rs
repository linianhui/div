extern crate div;
use div::class::ModifiedUtf8;

#[test]
fn test_to_string() {
    let string = ModifiedUtf8::to_string();

    assert_eq!("string", string);
}

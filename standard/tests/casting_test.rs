#[test]
fn casting_to_unsigned() {
    assert_eq!(1i32 as u32, 1u32);
    assert_eq!(i32::max_value() as u32, u32::max_value() >> 1);
    assert_eq!(-1i32 as u32, u32::max_value());
    assert_eq!(-2i32 as u32, u32::max_value() - 1);
    assert_eq!(-3i32 as u32, u32::max_value() - 2);
}

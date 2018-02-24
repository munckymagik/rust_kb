#[test]
fn compile_time_type_enforcement() {
    fn is_convertable<T: Into<i64>>(_n: T) {}

    is_convertable(1);
    is_convertable(1i8);
    is_convertable(1i16);
    is_convertable(1i32);
    is_convertable(1i64);
    is_convertable(1u8);
    is_convertable(1u16);
    is_convertable(1u32);

    // Won't compile
    // is_convertable(1u64);
    // ^^^^^^^^^^^^^^ the trait `std::convert::From<u64>` is not implemented for `i64`
}

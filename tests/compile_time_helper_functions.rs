#[test]
fn compile_time_conversion_checking() {
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

#[test]
fn compile_time_marker_trait_checking() {
    trait MarkerTrait {};

    // Compile time type checker helper function
    fn is_ok<A: MarkerTrait>(_: A) {}

    struct WrapperOne<A>(A);
    impl<A> MarkerTrait for WrapperOne<A> {};
    is_ok(WrapperOne(1));

    // struct WrapperTwo<A>(A);
    // Won't compile!
    // is_ok(WrapperTwo(1));
    // ^^^^^ the trait `compile_time_marker_trait_checking::MarkerTrait` is not implemented for
    //       `compile_time_marker_trait_checking::WrapperTwo<{integer}>`
}

#[test]
fn compile_time_send_safe_checking() {
    trait MarkerTrait<A> {};

    struct WrapperOne<A>(A);
    impl<A> MarkerTrait<A> for WrapperOne<A> {};

    struct WrapperTwo<A>(A);

    // Compile time type checker helper function
    fn is_ok<A, B>(_: B)
    where
        A: Send + 'static,
        B: MarkerTrait<A>,
    {
    }

    is_ok(WrapperOne(1));
    // is_ok(WrapperTwo(1));
    // ^^^^^ the trait `compile_time_send_safe_checking::MarkerTrait<_>` is not implemented for
    //       `compile_time_send_safe_checking::WrapperTwo<{integer}>`

    is_ok(WrapperOne("Hello"));
    is_ok(WrapperOne(Box::new(1)));

    // use std::rc::Rc;
    // is_ok(WrapperOne(Rc::new(5)));
    // ^^^^^ `std::rc::Rc<{integer}>` cannot be sent between threads safely

    use std::sync::Arc;
    is_ok(WrapperOne(Arc::new(5)));
}

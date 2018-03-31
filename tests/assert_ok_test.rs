#[macro_use]
mod assert_panics;
use std::fmt::Display;

macro_rules! assert_ok {
    ($e:expr) => (
        match $e {
            Ok(_) => (),
            Err(e) => panic!("`{}` failed with: {}", stringify!($e), e),
        }
    )
}

fn fn_assert_ok<T, E: Display>(actual: Result<T, E>) {
    match actual {
        Ok(_) => (),
        Err(e) => panic!("Expected Ok value but was Err with: {}", e),
    }
}

fn return_ok() -> Result<i32, i32> {
    Ok(1)
}
fn return_err() -> Result<i32, &'static str> {
    Err("Oh no!")
}

#[test]
fn test_assert_ok_ok_examples() {
    fn_assert_ok(return_ok());
    assert_ok!(return_ok());
}

#[test]
fn test_assert_ok_err_examples() {
    assert_panics!(
        fn_assert_ok(return_err()),
        "Expected Ok value but was Err with: Oh no!"
    );
    // ---- assert_ok_examples stdout ----
    //      thread 'assert_ok_examples' panicked at 'Expected Ok value but was Err with: Oh no!',
    //      tests/reducing_duplication.rs:14:17

    assert_panics!(
        assert_ok!(return_err()),
        "`return_err()` failed with: Oh no!"
    );
    // ---- assert_ok_examples stdout ----
    //      thread 'assert_ok_examples' panicked at 'assert_ok!(return_err()) failed with: Oh no!',
    //      tests/reducing_duplication.rs:14:3
}

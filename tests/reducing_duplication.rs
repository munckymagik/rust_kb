use std::panic;
use std::fmt::Display;

macro_rules! assert_ok {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("assert_ok!({}) failed with: {}", stringify!($e), e),
    })
}

macro_rules! assert_panics {
    ($panicking_expr:expr, $expected_cause:expr) => {
        {
            let result = panic::catch_unwind(|| {
                $panicking_expr
            });
            match result {
                Ok(_) => panic!("`{}` did not cause an error", stringify!($panicking_expr)),
                Err(ref boxed_any) => {
                    let cause = if let Some(&str_slice) = boxed_any.downcast_ref::<&str>() {
                                    str_slice
                                } else if let Some(string) = boxed_any.downcast_ref::<String>() {
                                    &string
                                } else {
                                    panic!("Cause of panic is not a String or a &str");
                                };

                    assert_eq!(cause, $expected_cause);
                }
            }
        }
    }
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
        "assert_ok!(return_err()) failed with: Oh no!"
    );
    // ---- assert_ok_examples stdout ----
    //      thread 'assert_ok_examples' panicked at 'assert_ok!(return_err()) failed with: Oh no!',
    //      tests/reducing_duplication.rs:14:3
}

#[test]
fn assert_panic_unwind_example() {
    assert_panics!(panic!("oh my god"), "oh my god");
    assert_panics!(
        Err::<(), &str>("something").unwrap(),
        "called `Result::unwrap()` on an `Err` value: \"something\""
    );
    assert_panics!(
        panic!("oh my god, {} was not {}", 1, 2),
        "oh my god, 1 was not 2"
    );
    assert_panics!(
        assert_ok!(return_err()),
        "assert_ok!(return_err()) failed with: Oh no!"
    );

    assert_panics!(
        assert_panics!((), "Expecting some error that won't happen"),
        "`()` did not cause an error"
    );

    assert_panics!(
        assert_panics!(
            assert_ok!(return_ok()),
            "Expecting some error that won't happen"
        ),
        "`assert_ok!(return_ok (  ))` did not cause an error"
    );
}

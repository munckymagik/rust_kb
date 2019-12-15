#[macro_use]
mod assert_panics;

#[test]
fn test_assert_panics_with_str_panic_objects() {
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
        assert_panics!((), "Expecting some error that won't happen"),
        "`()` did not cause an error"
    );

    assert_panics!(
        assert_panics!(1 + 1, "Expecting some error that won't happen"),
        "`1 + 1` did not cause an error"
    );

    assert_panics!(
        assert_panics!({ () }, "Expecting some error that won't happen"),
        "`{ () }` did not cause an error"
    );
}

#[test]
fn test_assert_panics_with_non_str_panic_objects() {
    #[derive(PartialEq, Debug)]
    struct NotAString;

    assert_eq!(NotAString, NotAString);
    assert_panics!(panic!(NotAString), NotAString, NotAString);

    assert_panics!(
        assert_panics!(panic!(NotAString), "oh my god"),
        "Cause of panic is not a String or a &str"
    );

    type TempResult<'a> = Result<(), &'a str>;
    fn err(msg: &str) -> TempResult<'_> {
        Err(msg)
    }

    assert_panics!(panic!(err("whatever")), err("whatever"), TempResult<'_>);
}

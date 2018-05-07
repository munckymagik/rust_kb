#[macro_use]
mod assert_panics;

macro_rules! debug_assert_no_trailing_nul {
    ($str_like:expr) => {
        debug_assert!(
            $str_like.as_bytes().last().map_or(true, |c| *c != b'\0'),
            "{:?} had a trailing nul",
            $str_like
        );
    };
}

#[test]
fn ensure_no_trailing_nul() {
    let a: String = "hello".to_string();
    debug_assert_no_trailing_nul!(a);

    let b: &str = "";
    debug_assert_no_trailing_nul!(b);

    let c = "hello\0";
    assert_panics!(
        debug_assert_no_trailing_nul!(c),
        "\"hello\\u{0}\" had a trailing nul"
    );

    println!("{}", b);
}

// A file scope shared helper
fn assert_empty<T: IntoIterator, F: Fn() -> T>(f: F) {
    assert!(f().into_iter().count() == 0);
}

#[test]
fn test_empty() {
    // A function scope factory function
    fn empty() -> Vec<i32> { vec![] }

    assert_empty(|| empty());
    assert_empty(|| {
        let mut e = empty();
        e.append(&mut empty());
        e
    });
}

#[test]
fn test_empty_with_join() {
    // A function scope factory function
    fn empty() -> Vec<u64> { vec![] }

    // A function scope helper
    fn join(mut a: Vec<u64>, mut b: Vec<u64>) -> Vec<u64> {
        a.append(&mut b);
        a
    }

    assert_empty(|| empty());
    assert_empty(|| join(empty(), empty()));
}

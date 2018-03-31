use std::panic;

macro_rules! assert_ok {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("assert_ok!({}) failed with: {}", stringify!($e), e),
    })
}

fn return_ok() -> Result<i32, i32> { Ok(1) }
fn return_err() -> Result<i32, &'static str> { Err("Oh no!") }

#[test]
fn assert_ok_examples() {
  assert_ok!(return_ok());
  // assert_ok!(return_err());
  // ---- assert_ok_examples stdout ----
  //      thread 'assert_ok_examples' panicked at 'assert_ok!(return_err()) failed with: Oh no!', tests/reducing_duplication.rs:14:3
}

#[test]
fn assert_panic_unwind_example() {
  let result = panic::catch_unwind(|| {
      assert_ok!(return_err());
  });
  match result {
    Ok(_) => panic!("Was not an error"),
    Err(ref boxed_any) => {
      match boxed_any.downcast_ref::<String>() {
        Some(cause) => assert_eq!(cause, "assert_ok!(return_err()) failed with: Oh no!"),
        None        => panic!("Cause is not a String")
      }
    }
  }
}
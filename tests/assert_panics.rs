#[nowarn(unused_macros)]

macro_rules! assert_panics {
    ($panicking_expr:expr, $expected_cause:expr) => {
        {
            use std::panic;
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
    };
    ($panicking_expr:expr, $expected_cause:expr, $cause_type:ty) => {
        {
            use std::panic;
            let result = panic::catch_unwind(|| {
                $panicking_expr
            });
            match result {
                Ok(_) => panic!("`{}` did not cause an error", stringify!($panicking_expr)),
                Err(ref boxed_any) => {
                    let cause: &$cause_type = boxed_any
                        .downcast_ref::<$cause_type>()
                        .expect(&format!("Cause of panic is not a {}", stringify!($cause_type)));

                    assert_eq!(cause, &$expected_cause);
                }
            }
        }
    }
}

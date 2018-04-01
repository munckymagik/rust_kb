#[test]
fn test_context_providing_macros() {
    assert_eq!(line!(), 3);
    assert_eq!(column!(), 16);

    assert_eq!(file!(), "tests/context_providing_macros_test.rs");
    assert_eq!(module_path!(), "context_providing_macros_test");

    mod nested {
        pub fn test_module_path() {
            assert_eq!(module_path!(), "context_providing_macros_test::nested");
        }
    }

    nested::test_module_path();

    assert_eq!(stringify!(1 + 1), "1 + 1");
    assert_eq!(
        stringify!(some_imaginary_code()),
        "some_imaginary_code (  )"
    );
    assert_eq!(
        stringify!(struct Whatever<'a> {
            i: i32,
            s: &'a str
        }),
        "struct Whatever < 'a > { i : i32 , s : & 'a str }"
    );
}

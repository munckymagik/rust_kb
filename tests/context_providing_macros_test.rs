#[test]
fn test_file_line_column_module() {
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
}

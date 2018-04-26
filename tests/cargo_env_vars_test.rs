static STATICALLY_CONCATED_STR: &str = concat!(
    env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION")
);

#[test]
fn test_statically_concat_env_vars() {
    assert_eq!(
        STATICALLY_CONCATED_STR,
        "testing_rust 0.1.0"
    );
}

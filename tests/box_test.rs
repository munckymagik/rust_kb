#[test]
fn it_prevents_the_need_for_copying() {
    let a = Box::new(5);

    fn get_addr<T: AsRef<i32>>(b: T) -> *const i32 {
        b.as_ref()
    }

    let addr_1 = get_addr(&a);
    let addr_2 = get_addr(a);

    assert_eq!(addr_1, addr_2);
}

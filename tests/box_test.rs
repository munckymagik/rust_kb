#[test]
fn example_transferring_ownership_without_copying() {
    // 5 is arbitrary, the real contained value could be a huge chunk of memory that
    // must not be copied.
    let a = Box::new(5);

    // Using AsRef allows us to pass either a reference to the Box or move it in
    fn get_addr<T: AsRef<i32>>(b: T) -> *const i32 {
        // Returns a reference to the inner value
        b.as_ref()
    }

    // First we pass in a reference to Box and get the address of the inner value
    let addr_1 = get_addr(&a);
    // Next we move the box in and again we get the address of the inner value
    let addr_2 = get_addr(a);

    // In both cases the address of the contained objects remains constant
    assert_eq!(addr_1, addr_2);
}

mod type_size_not_known_at_compile_time {
    // See https://doc.rust-lang.org/book/second-edition/ch15-01-box.html#enabling-recursive-types-with-boxes
    // Without wrapping in a Box, nesting List in Cons will cause an infinitely sized type
    // Box however, has a size known at compile time
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    #[test]
    fn example_recursive_type() {
        use self::List::{Cons, Nil};

        let _list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    }

    #[test]
    fn example_returning_closure_from_a_function() {
        // See: https://doc.rust-lang.org/book/first-edition/closures.html#returning-closures
        fn return_a_closure() -> Box<Fn(i32) -> i32> {
            let num = 5;
            Box::new(move |x| x + num)
        }

        let f = return_a_closure();
        let answer = f(1);
        assert_eq!(answer, 6);
    }
}

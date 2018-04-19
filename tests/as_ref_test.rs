#[test]
fn it_provides_a_way_to_pass_many_types_in() {
    struct A(i32);
    struct B(i32);

    impl AsRef<i32> for A {
        fn as_ref(&self) -> &i32 {
            &self.0
        }
    }

    impl AsRef<i32> for B {
        fn as_ref(&self) -> &i32 {
            &self.0
        }
    }

    fn wants_i32<T: AsRef<i32>>(t: &T) -> &i32 {
        t.as_ref()
    }

    let a = A(42);
    assert_eq!(wants_i32(&a), &42);

    let b = B(43);
    assert_eq!(wants_i32(&b), &43);
}

#[test]
fn it_can_be_used_with_smart_pointers() {
    struct SmartPointer(i32);

    impl std::ops::Deref for SmartPointer {
        type Target = i32;

        fn deref(&self) -> &i32 {
            &self.0
        }
    }

    impl AsRef<i32> for SmartPointer {
        fn as_ref(&self) -> &i32 {
            // &self _is_ &i32 because we implement Deref
            // This example is from String
            self
        }
    }

    // Note: we don't need the caller to pass in a reference
    fn wants_i32<T: AsRef<i32>>(t: T) -> i32 {
        *t.as_ref()
    }

    let b = SmartPointer(43);
    assert_eq!(wants_i32(b), 43);
}

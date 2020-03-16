mod borrowing {
    #[test]
    fn can_use_previous_binding_after_passing_to_function_as_ref() {
        #[allow(clippy::ptr_arg)]
        fn borrow_ownership(_v: &Vec<i64>) {
            // References are immutable too ...
            // v.push(4);
            // error: cannot borrow immutable borrowed content `*v` as mutable

            // does not deallocate v as it is borrowed as a reference
        }

        let v = vec![1, 2, 3];
        assert_eq!(v[0], 1);
        borrow_ownership(&v); // Borrow happens when we pass a reference to v
        assert_eq!(v[0], 1); // We can still use the original binding
    }
}

mod mutable_references {
    #[test]
    fn mutable_references_allow_modification_of_a_borrowed_resource() {
        // Must receive param as explicitly mutable
        fn borrow_and_mutate(v: &mut Vec<i64>) {
            v.push(4);
            // does not deallocate v as it is borrowed as a reference
        }

        let mut v = vec![1, 2, 3]; // Must bind as mutable
        assert_eq!(v.len(), 3);
        borrow_and_mutate(&mut v); // Must pass as explicitly mutable
        assert_eq!(v.len(), 4); // We can still use the original binding
    }

    #[test]
    fn use_the_asterisk_to_access_the_content_of_the_mutable_reference() {
        let mut x = 5;
        assert_eq!(x, 5);
        {
            let y = &mut x;

            // Asterisk needed to access the contents of y
            //assert_eq!(*y, 5);
            // error: the trait bound `&mut _: std::cmp::PartialEq<_>` is not satisfied [E0277]

            // Asterisk needed to apply the plus-equals operator on y
            //y += 1;
            // error: binary assignment operation `+=` cannot be applied to type `&mut _` [E0368]
            *y += 1;
        }

        // y is out of scope, so there are 0 mutable references to x and we can use it again
        assert_eq!(x, 6);
    }
}

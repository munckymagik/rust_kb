mod move_semantics {
    #[test]
    fn cant_use_previous_binding_after_reassignment() {
        let v = vec![1, 2, 3];
        assert_eq!(v[0], 1);
        let _v2 = v; // Move happens when we assign it to a new binding
        //assert_eq!(v[0], 1);
        // error: use of moved value: `v` [E0382]
        // note: `v` moved here because it has type `std::vec::Vec<i32>`, which is moved by default
    }

    #[test]
    fn cant_use_previous_binding_after_passing_to_function() {
        fn take_ownership(_v: Vec<i64>) {
            // will deallocate _v at the end of its scope as it has taken ownership
        }

        let v = vec![1, 2, 3];
        assert_eq!(v[0], 1);
        take_ownership(v); // Move happens when we assign it to a new binding
        //assert_eq!(v[0], 1);
        // error: use of moved value: `v` [E0382]
        // note: `v` moved here because it has type `std::vec::Vec<i64>`, which is non-copyable
    }

    #[test]
    fn handing_back_is_possible_but_tedious() {
        fn take_ownership_then_give_back(v: Vec<i64>) -> Vec<i64> {
            // Pretend to do something with v here
            v // Return v to pass it back
        }

        let v = vec![1, 2, 3];
        assert_eq!(v[0], 1);
        let v = take_ownership_then_give_back(v);
        assert_eq!(v[0], 1);

        // A better solution is 'borrowing'
    }
}

mod copy_types {
    #[test]
    fn primitives_are_copied_on_assignment() {
        let v = 1;
        assert_eq!(v, 1);
        let _v2 = v;
        assert_eq!(v, 1);
    }

    #[test]
    fn primitives_are_copied_on_passing() {
        fn do_something_with_arg(v: i64) -> i64 {
            v * 2
        }

        let v = 1;
        assert_eq!(v, 1);
        let _v2 = do_something_with_arg(v);
        assert_eq!(v, 1);
    }
}

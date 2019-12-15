use std::borrow::Cow;

#[test]
fn it_is_a_smart_pointer() {
    // You create a Cow from some data
    let a = [1, 2, 3];
    let a_cow = Cow::from(&a[..]);

    // It forwards method calls to the data inside
    assert_eq!(a_cow.len(), 3);
    assert_eq!(a_cow[0], 1);
    assert_eq!(a_cow[1], 2);

    // You can create slices from the data inside
    let _x: &[i32] = &a_cow[..];

    // The data is wrapped in a Cow::Borrowed
    if let Cow::Borrowed(ref_a) = a_cow {
        // The data inside points to the same memory as our original variable
        let p_a: *const [i32] = &a;
        let p_a_from_cow = ref_a as *const [i32];
        assert_eq!(p_a, p_a_from_cow);
    } else {
        panic!("Not a Cow::Borrowed");
    }
}

#[test]
fn it_makes_a_copy_when_we_write_to_the_wrapped_data() {
    // The original value does not have to be mutable
    let a = [1, 2, 3];
    let mut a_cow = Cow::from(&a[..]);

    // You need to call `.to_mut` to modify the contents
    a_cow.to_mut()[0] = 9;

    // Now element 0 in the Cow is 9
    assert_eq!(a_cow[0], 9);

    // Now the Cow has mutated itself into a Cow::Owned
    if let Cow::Owned(a_copy) = a_cow {
        // The data has been copied into a Vec, which is the <i32 as ToOwned>::Owned type
        let _x: &Vec<i32> = &a_copy;

        // and the memory pointed to is no longer the same as `a`
        let p_a_0: *const i32 = &a[0];
        let p_a_0_copy = &a_copy[0] as *const i32;

        assert_ne!(p_a_0, p_a_0_copy);
    } else {
        panic!("Not a Cow::Owned");
    }
}

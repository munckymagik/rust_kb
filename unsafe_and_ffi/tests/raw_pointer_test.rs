// References:
// * https://doc.rust-lang.org/book/first-edition/raw-pointers.html
// * https://doc.rust-lang.org/std/primitive.pointer.html
// * https://doc.rust-lang.org/std/ptr/fn.null.html

#![allow(clippy::float_cmp)]

#[test]
fn test_refs_cast_to_pointers_automatically() {
    fn expects_a_pointer(pa: *const u32) {
        unsafe {
            assert_eq!(*pa, 42);
        }
    }

    let a = 42u32;

    expects_a_pointer(&a);
}

#[test]
fn test_mut_pointer_arg() {
    fn mutate_arg(pa: *mut u32) {
        unsafe { *pa += 1 };
    }

    let mut a = 42u32;
    mutate_arg(&mut a);
    assert_eq!(a, 43);
}

#[test]
fn test_from_const_pointer_to_ref() {
    fn to_ptr_and_back<'a>(pa: *const u32) -> &'a u32 {
        unsafe { &*pa }
    }

    let a: u32 = 42;
    assert_eq!(to_ptr_and_back(&a), &42);
}

#[test]
fn test_pointer_aritmetic() {
    fn get_second_element(pa: *const [u32; 2]) -> u32 {
        unsafe {
            // Trick the compiler into allowing ptr arithmetic by casting to usize and back
            let mut address = pa as usize;
            address += 4;
            let pb = address as *const [u32; 1];
            (*pb)[0]
        }
    }

    let a = [42, 43];
    let b = get_second_element(&a);
    assert_eq!(b, 43);
}

#[test]
fn test_pointer_offset() {
    fn unsafe_index<T: Copy>(pa: *const T, index: isize) -> T {
        unsafe { *pa.offset(index) }
    }

    let a = [42, 43];
    assert_eq!(unsafe_index(a.as_ptr(), 0), 42);
    assert_eq!(unsafe_index(a.as_ptr(), 1), 43);

    let a = [1.0, 2.0];
    assert_eq!(unsafe_index(a.as_ptr(), 0), 1.0);
    assert_eq!(unsafe_index(a.as_ptr(), 1), 2.0);
}

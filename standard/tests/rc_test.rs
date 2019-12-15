use std::rc::Rc;

fn consume_rc(rc: Rc<&[u8]>) -> usize {
    Rc::strong_count(&rc)
}

#[test]
fn test_needs_explicit_clone() {
    let rc = Rc::new(&b"some bytes"[..]);

    // We have one Rc to start with
    assert_eq!(Rc::strong_count(&rc), 1);

    // We clone and pass immediately to the function consume_rc
    // Note we have to explicitly clone, passing an Rc by value does not trigger
    // a clone, it moves ownership of the Rc instead.
    // Expect two alive references, one in this scope the other was in the function
    assert_eq!(consume_rc(Rc::clone(&rc)), 2);

    // The reference in the function is out of scope, so we should be back to 1
    assert_eq!(Rc::strong_count(&rc), 1);
}

#[test]
fn test_borrow_references_have_no_effect_on_reference_count() {
    let rc = Rc::new(&b"some bytes"[..]);

    // Borrowing the Rc has no effect on the actual reference count
    let rc_borrow = &rc;
    assert_eq!(Rc::strong_count(&rc_borrow), 1);
}

#[test]
fn test_passing_by_value_consumes_a_reference() {
    let rc = Rc::new(&b"some bytes"[..]);

    // Create a second reference
    let rc2 = Rc::clone(&rc);

    // We expect two references
    assert_eq!(Rc::strong_count(&rc), 2);

    // Let our second reference be consumed by the function
    assert_eq!(consume_rc(rc2), 2);

    // Now we're back to 1 reference
    assert_eq!(Rc::strong_count(&rc), 1);
}

#[test]
fn test_drop_explicitly_reduces_reference_count() {
    let rc = Rc::new(&b"some bytes"[..]);

    // Create a second reference
    let rc2 = Rc::clone(&rc);

    // We expect two references
    assert_eq!(Rc::strong_count(&rc), 2);

    // Explicitly drop out second reference
    drop(rc2);

    // Now we're back to 1 reference
    assert_eq!(Rc::strong_count(&rc), 1);
}

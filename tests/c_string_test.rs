use std::ffi::{
    CStr,
};

#[test]
fn when_there_is_a_trailing_nul() {
    let bytes = b"hello\0";
    assert_eq!(&[104, 101, 108, 108, 111, 0], bytes);

    let cstr = CStr::from_bytes_with_nul(bytes).unwrap();
    assert_eq!(format!("{:?}", cstr), "\"hello\"");
}

#[test]
fn when_there_is_no_trailing_nul() {
    let bytes = b"hello";
    assert_eq!(&[104, 101, 108, 108, 111], bytes);

    // NotNulTerminated error
    assert!(CStr::from_bytes_with_nul(bytes).is_err());
}

#[test]
fn when_there_are_2_trailing_nuls() {
    let bytes = b"hello\0\0";
    assert_eq!(&[104, 101, 108, 108, 111, 0, 0], bytes);

    // Will be an InteriorNul error
    assert!(CStr::from_bytes_with_nul(bytes).is_err());

    let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(bytes) };
    // Extra nul is retained!!
    assert_eq!(format!("{:?}", cstr), "\"hello\\x00\"");

    // First we need to find the first nul
    let nul_pos = bytes.iter().position(|elem| *elem == b'\0').unwrap();
    assert_eq!(nul_pos + 2, bytes.len());
    // Then adjust our slice
    let bytes_trimmed = &bytes[..(nul_pos + 1)];
    // Now we can create a CStr safely
    let cstr = CStr::from_bytes_with_nul(bytes_trimmed).unwrap();
    // And sanity is restored
    assert_eq!(format!("{:?}", cstr), "\"hello\"");
}

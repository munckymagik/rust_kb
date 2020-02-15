// Experimenting with safe-only alternatives for a BStr e.g.
//
// Useful links:
// * https://github.com/BurntSushi/bstr/blob/master/src/bstr.rs
// * https://docs.rs/bstr/0.2.11/bstr/fn.B.html for why we need as_ref()
// * https://users.rust-lang.org/t/how-to-create-a-slice-like-type/29073/3

use std::borrow::Cow;

#[derive(Debug, PartialEq)]
struct BStr<'inner> {
    bytes: &'inner [u8],
}

impl<'outer> BStr<'outer> {
    fn new<T>(other: &'outer T) -> Self
    where
        T: AsRef<[u8]> + ?Sized
    {
        BStr { bytes: other.as_ref() }
    }

    fn to_str_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.bytes)
    }
}

impl<'outer, T> From<&'outer T> for BStr<'outer>
where
    T: AsRef<[u8]> + ?Sized
{
    fn from(other: &'outer T) -> Self {
        BStr { bytes: other.as_ref() }
    }
}

#[test]
fn test_bstr_size() {
    assert_eq!(std::mem::size_of::<&[u8]>(), 16);
    assert_eq!(std::mem::size_of::<BStr>(), 16);
}
#[test]
fn test_bstr_new() {
    let a = BStr::new(b"hello");
    let b = BStr::new("hello");
    assert_eq!(a, b);
}

#[test]
fn test_bstr_from() {
    let a = BStr::from(b"hello");
    let b = BStr::from("hello");
    assert_eq!(a, b);
}

#[test]
fn test_bstr_to_str_lossy() {
    let a = BStr::from("hello");
    assert_eq!(a.to_str_lossy(), "hello");
}

// Experimenting with safe-only alternatives for a BStr e.g.
//
// Useful links:
// * https://github.com/BurntSushi/bstr/blob/master/src/bstr.rs
// * https://docs.rs/bstr/0.2.11/bstr/fn.B.html for why we need as_ref()
// * https://users.rust-lang.org/t/how-to-create-a-slice-like-type/29073/3

use std::borrow::Cow;
use std::fmt::{self, Display};

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

    fn eq_ignore_ascii_case<T>(&self, other: T) -> bool
    where
        T: AsRef<[u8]>
    {
        self.bytes.eq_ignore_ascii_case(other.as_ref())
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

impl AsRef<[u8]> for BStr<'_> {
    fn as_ref(&self) -> &[u8] {
        self.bytes
    }
}

impl Display for BStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_str_lossy())
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

#[test]
fn test_bstr_display() {
    use std::fmt::Write;

    let a = BStr::from("hello");
    let mut buf = String::new();

    write!(buf, "{}", a).unwrap();

    assert_eq!(buf, "hello")
}

#[test]
fn test_bstr_eq_ignore_ascii_case() {
    let a = BStr::from("hEllo");
    let b = BStr::from("helLo");

    assert!(a.eq_ignore_ascii_case(&b));
    assert!(a.eq_ignore_ascii_case(b"HeLlO"));
    assert!(a.eq_ignore_ascii_case("HeLlO"));
}

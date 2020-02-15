// Experimenting with alternatives for a BStr e.g.
// https://github.com/BurntSushi/bstr/blob/master/src/bstr.rs
// See https://docs.rs/bstr/0.2.11/bstr/fn.B.html for why we need as_ref()

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
fn test_bstr() {
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

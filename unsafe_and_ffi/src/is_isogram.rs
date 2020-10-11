//! From the exercism.io exercise:
//! https://exercism.io/my/solutions/41dd9bf15c31491cb9260ae926c4d9b2

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

const NUM_ALPHAS: usize = 26;
const NUL: c_char = b'\0' as c_char;

mod ext {
    use std::os::raw::c_int;
    extern "C" {
        pub fn tolower(c: c_int) -> c_int;
        pub fn isalpha(c: c_int) -> c_int;
    }
}

// To avoid dead code warning without exposing publicly from module
use ext::*;

pub fn is_isogram(phrase: &CStr) -> bool {
    let mut counts = [0; NUM_ALPHAS];

    let mut it = phrase.as_ptr();
    unsafe {
        while *it != NUL {
            let c = tolower(*it as c_int);

            if isalpha(c) > 0 {
                let char_key = (c - (b'a' as c_int)) as usize;

                if counts[char_key] > 0 {
                    return false;
                }

                counts[char_key] += 1;
            }

            it = it.add(1);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_empty_string()
    {
       assert!(is_isogram(&CString::new("").unwrap()));
    }

    #[test]
    fn test_lower_case_only()
    {
       assert!(is_isogram(&CString::new("isogram").unwrap()));
    }

    #[test]
    fn test_duplicated_letter()
    {
       assert!(!is_isogram(&CString::new("eleven").unwrap()));
    }

    #[test]
    fn test_longest_known_isogram()
    {
       assert!(is_isogram(&CString::new("subdermatoglyphic").unwrap()));
    }

    #[test]
    fn test_duplicated_letter_mixed_case()
    {
       assert!(!is_isogram(&CString::new("Alphabet").unwrap()));
    }

    #[test]
    fn test_non_letter_char()
    {
       assert!(is_isogram(&CString::new("thumbscrew-japingly").unwrap()));
    }

    #[test]
    fn test_duplicated_non_letter_char()
    {
       assert!(is_isogram(&CString::new("Hjelmqvist-Gryb-Zock-Pfund-Wax").unwrap()));
    }

    #[test]
    fn test_multiple_whitespace()
    {
       assert!(is_isogram(&CString::new("Emily Jung Schwartzkopf").unwrap()));
    }

    #[test]
    fn test_duplicated_letter_within_word()
    {
       assert!(!is_isogram(&CString::new("accentor").unwrap()));
    }
}

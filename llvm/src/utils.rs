use std::borrow::Cow;
use std::ffi::{CStr, CString};

pub fn to_c_str<'s>(mut s: &'s str) -> Cow<'s, CStr> {
    if s.is_empty() {
        s = "\0";
    }

    // Start from the end of the string as it's the most likely place to find a null byte
    if s.chars().rev().find(|&ch| ch == '\0').is_none() {
        return Cow::from(CString::new(s).expect("Unreachable since null bytes are checked"));
    }

    unsafe { Cow::from(CStr::from_ptr(s.as_ptr() as *const _)) }
}

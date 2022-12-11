use libc::c_char;
use llvm_sys::core::*;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::fmt;
use std::ops::Deref;

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

/// An owned LLVM String. Also known as a LLVM Message
#[derive(Eq)]
pub struct LLVMString {
    pub(crate) ptr: *const c_char,
}

impl LLVMString {
    pub(crate) unsafe fn new(ptr: *const c_char) -> Self { LLVMString { ptr } }

    pub fn to_string(&self) -> String { (*self).to_string_lossy().into_owned() }

    pub(crate) fn create_from_c_str(string: &CStr) -> LLVMString {
        unsafe { LLVMString::new(LLVMCreateMessage(string.as_ptr() as *const _)) }
    }

    pub(crate) fn create_from_str(string: &str) -> LLVMString {
        debug_assert_eq!(string.as_bytes()[string.as_bytes().len() - 1], 0);

        unsafe { LLVMString::new(LLVMCreateMessage(string.as_ptr() as *const _)) }
    }
}

impl Deref for LLVMString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target { unsafe { CStr::from_ptr(self.ptr) } }
}

impl fmt::Debug for LLVMString {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> { write!(f, "{:?}", self.deref()) }
}

impl fmt::Display for LLVMString {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> { write!(f, "{:?}", self.deref()) }
}

impl PartialEq for LLVMString {
    fn eq(&self, other: &LLVMString) -> bool { **self == **other }
}

// impl Error for LLVMString {
//     fn description(&self) -> &str {
//         self.to_str()
//             .expect("Could not convert LLVMString to str (likely invalid unicode)")
//     }

//     fn cause(&self) -> Option<&dyn Error> { None }
// }

impl Drop for LLVMString {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeMessage(self.ptr as *mut _);
        }
    }
}

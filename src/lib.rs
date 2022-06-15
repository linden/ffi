#![feature(core_c_str)]

use std::ffi::CString;
use core::ffi::{CStr};

#[no_mangle]
pub unsafe extern "C" fn create_string(source: *mut libc::c_char) -> Box<String> {
    let cstring: &CStr = CStr::from_ptr(source);
    let string: &str = cstring.to_str().unwrap();
    let buffer: String = string.to_owned();

    Box::new(buffer)
}

#[no_mangle]
pub unsafe extern "C" fn create_cstring(source: Box<String>) -> *mut libc::c_char {
    let string = *source;
    let cstring = CString::new(string).unwrap();

    cstring.into_raw()
}

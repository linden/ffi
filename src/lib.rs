#![feature(core_c_str)]

use std::slice;
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

#[no_mangle]
pub unsafe extern "C" fn create_str(source: *mut libc::c_char) -> Box<&'static str> {
    let cstring: &CStr = CStr::from_ptr(source);
    let string: &str = cstring.to_str().unwrap();
    let buffer: String = string.to_owned();

    Box::new(Box::leak(buffer.into_boxed_str()))
}

#[no_mangle]
pub unsafe extern "C" fn create_cstr(source: Box<&str>) -> *mut libc::c_char {
    let string = *source;
    let cstring = CString::new(string).unwrap();

    cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn create_from_u8(source: Box<u8>) -> *mut libc::c_uint {
    (*source) as *mut libc::c_uint
}

#[no_mangle]
pub extern "C" fn create_u8(source: *mut libc::c_uint) -> Box<u8> {
    Box::new(source as u8)
}

#[no_mangle]
pub extern "C" fn create_from_u16(source: Box<u16>) -> *mut libc::c_uint {
    (*source) as *mut libc::c_uint
}

#[no_mangle]
pub extern "C" fn create_u16(source: *mut libc::c_uint) -> Box<u16> {
    Box::new(source as u16)
}

#[no_mangle]
pub extern "C" fn create_from_u32(source: Box<u32>) -> *mut libc::c_uint {
    (*source) as *mut libc::c_uint
}

#[no_mangle]
pub extern "C" fn create_u32(source: *mut libc::c_uint) -> Box<u32> {
    Box::new(source as u32)
}

#[no_mangle]
pub extern "C" fn create_from_u64(source: Box<u64>) -> *mut libc::c_ulong {
    (*source) as *mut libc::c_ulong
}

#[no_mangle]
pub extern "C" fn create_u64(source: *mut libc::c_ulong) -> Box<u64> {
    Box::new(source as u64)
}

#[no_mangle]
pub extern "C" fn create_from_usize(source: Box<usize>) -> *mut libc::size_t {
    (*source) as *mut libc::size_t
}

#[no_mangle]
pub extern "C" fn create_usize(source: *mut libc::size_t) -> Box<usize> {
    Box::new(source as usize)
}

#[no_mangle]
pub extern "C" fn create_from_i8(source: Box<i8>) -> *mut libc::c_int {
    (*source) as *mut libc::c_int
}

#[no_mangle]
pub extern "C" fn create_i8(source: *mut libc::c_int) -> Box<i8> {
    Box::new(source as i8)
}

#[no_mangle]
pub extern "C" fn create_from_i16(source: Box<i16>) -> *mut libc::c_int {
    (*source) as *mut libc::c_int
}

#[no_mangle]
pub extern "C" fn create_i16(source: *mut libc::c_int) -> Box<i16> {
    Box::new(source as i16)
}

#[no_mangle]
pub extern "C" fn create_from_i32(source: Box<i32>) -> *mut libc::c_int {
    (*source) as *mut libc::c_int
}

#[no_mangle]
pub extern "C" fn create_i32(source: *mut libc::c_int) -> Box<i32> {
    Box::new(source as i32)
}

#[no_mangle]
pub extern "C" fn create_from_i64(source: Box<i64>) -> *mut libc::c_long {
    (*source) as *mut libc::c_long
}

#[no_mangle]
pub extern "C" fn create_i64(source: *mut libc::c_long) -> Box<i64> {
    Box::new(source as i64)
}

#[no_mangle]
pub extern "C" fn create_from_isize(source: Box<isize>) -> *mut libc::ssize_t {
    (*source) as *mut libc::ssize_t
}

#[no_mangle]
pub extern "C" fn create_isize(source: *mut libc::ssize_t) -> Box<isize> {
    Box::new(source as isize)
}

#[no_mangle]
pub unsafe extern "C" fn create_u8_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [u8]> {
    Box::new(slice::from_raw_parts(source as *const u8, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_u16_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [u16]> {
    Box::new(slice::from_raw_parts(source as *const u16, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_u32_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [u32]> {
    Box::new(slice::from_raw_parts(source as *const u32, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_u64_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [u64]> {
    Box::new(slice::from_raw_parts(source as *const u64, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_usize_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [usize]> {
    Box::new(slice::from_raw_parts(source as *const usize, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_i8_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [i8]> {
    Box::new(slice::from_raw_parts(source as *const i8, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_i16_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [i16]> {
    Box::new(slice::from_raw_parts(source as *const i16, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_i32_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [i32]> {
    Box::new(slice::from_raw_parts(source as *const i32, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_i64_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [i64]> {
    Box::new(slice::from_raw_parts(source as *const i64, size as usize))
}

#[no_mangle]
pub unsafe extern "C" fn create_isize_array(source: *const libc::c_uint, size: libc::size_t) -> Box<&'static [isize]> {
    Box::new(slice::from_raw_parts(source as *const isize, size as usize))
}

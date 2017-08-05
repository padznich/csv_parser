extern crate libc;

use std::{slice, str};
use std::ffi::CStr;
use libc::{size_t, c_char};


pub fn transform_arr<'a>(array: *const *const c_char, length: size_t) -> Vec<&'a str> {
    let values = unsafe { slice::from_raw_parts(array, length as usize) };
    let strs: Vec<&str> = values.iter()
        .map(|&p| unsafe { CStr::from_ptr(p) })  // iterator of &CStr
        .map(|cs| cs.to_bytes())                 // iterator of &[u8]
        .map(|bs| str::from_utf8(bs).unwrap())   // iterator of &str
        .collect();
    strs
}

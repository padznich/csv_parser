extern crate csv;
extern crate libc;

use std::{iter, slice, str};
use std::ffi::{CStr, CString};
use libc::{size_t, c_char, uint32_t};

pub mod db;
pub mod parsers;
pub mod helpers;


#[no_mangle]
pub extern fn theme_song_generate(length: uint32_t) -> *mut c_char {
    let mut song = String::from("💣 ");
    song.extend(iter::repeat("na ").take(length as usize));
    song.push_str("Batman! 💣");

    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern fn theme_song_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern fn parse_many(array: *const *const c_char, length: size_t) {

    let values = unsafe { slice::from_raw_parts(array, length as usize) };
    let strs: Vec<&str> = values.iter()
        .map(|&p| unsafe { CStr::from_ptr(p) })  // iterator of &CStr
        .map(|cs| cs.to_bytes())                 // iterator of &[u8]
        .map(|bs| str::from_utf8(bs).unwrap())   // iterator of &str
        .collect();
    for f in strs {
        println!("{:?}", f);
    }
}

#[no_mangle]
pub extern fn parse(file_path: *const c_char, column_names: *const *const c_char, length: size_t) {

    let columns: Vec<&str> = helpers::trs::transform_arr(column_names, length);
    let csv_path = unsafe { CStr::from_ptr(file_path).to_str().unwrap() };

    let parsed_dict = parsers::parse_csv::parse_csv(csv_path, columns);

    db::psql::write(parsed_dict.unwrap());
    println!("Done");

}


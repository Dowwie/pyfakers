

use libc::c_char;
use std::str;
use std::ffi::CString;
use fake::faker::*;


#[no_mangle]
pub extern "C" fn full_name() -> *mut c_char {
    let _name = <Faker as Name>::name();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


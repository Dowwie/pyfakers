use libc::c_char;
use std::str;
use std::ffi::CString;
use fake::faker::*;


#[no_mangle]
pub extern "C" fn first_name() -> *mut c_char {
    let __name = <Faker as Name>::first_name();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn last_name() -> *mut c_char {
    let __name = <Faker as Name>::last_name();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn full_name() -> *mut c_char {
    let _name = <Faker as Name>::name();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn title_descriptor() -> *mut c_char {
    let __name = <Faker as Name>::title_descriptor();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn title_level() -> *mut c_char {
    let __name = <Faker as Name>::title_level();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn title_job() -> *mut c_char {
    let __name = <Faker as Name>::title_job();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn title() -> *mut c_char {
    let _name = <Faker as Name>::title();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


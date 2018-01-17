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



#[no_mangle]
pub extern "C" fn free_email_provider() -> *mut c_char {
    let _name = <Faker as Internet>::free_email_provider();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn domain_suffix() -> *mut c_char {
    let _name = <Faker as Internet>::domain_suffix();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn user_name() -> *mut c_char {
    let _name = <Faker as Internet>::user_name();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn free_email() -> *mut c_char {
    let _name = <Faker as Internet>::free_email();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn safe_email() -> *mut c_char {
    let _name = <Faker as Internet>::safe_email();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}



#[no_mangle]
pub extern "C" fn suffix() -> *mut c_char {
    let _name = <Faker as Company>::suffix();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn name() -> *mut c_char {
    let _name = <Faker as Company>::name();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn buzzword() -> *mut c_char {
    let _name = <Faker as Company>::buzzword();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn catch_phrase() -> *mut c_char {
    let _name = <Faker as Company>::catch_phase();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn bs() -> *mut c_char {
    let _name = <Faker as Company>::bs();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn profession() -> *mut c_char {
    let _name = <Faker as Company>::profession();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn industry() -> *mut c_char {
    let _name = <Faker as Company>::industry();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn time_zone() -> *mut c_char {
    let _name = <Faker as Address>::time_zone();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn city_prefix() -> *mut c_char {
    let _name = <Faker as Address>::city_prefix();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn city_suffix() -> *mut c_char {
    let _name = <Faker as Address>::city_suffix();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn street_suffix() -> *mut c_char {
    let _name = <Faker as Address>::street_suffix();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn state() -> *mut c_char {
    let _name = <Faker as Address>::state();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn state_abbr() -> *mut c_char {
    let _name = <Faker as Address>::state_abbr();
    let full_name = CString::new(_name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn city() -> *mut c_char {
    let _name = <Faker as Address>::city();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn street_name() -> *mut c_char {
    let _name = <Faker as Address>::street_name();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn building_number() -> *mut c_char {
    let _name = <Faker as Address>::building_number();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn street_address() -> *mut c_char {
    let _name = <Faker as Address>::street_address();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn secondary_address() -> *mut c_char {
    let _name = <Faker as Address>::secondary_address();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn postal_code() -> *mut c_char {
    let _name = <Faker as Address>::postcode();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn postcode() -> *mut c_char {
    let _name = <Faker as Address>::postcode();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn latitude() -> *mut c_char {
    let _name = <Faker as Address>::latitude();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}

#[no_mangle]
pub extern "C" fn longitude() -> *mut c_char {
    let _name = <Faker as Address>::longitude();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}


#[no_mangle]
pub extern "C" fn phone_number() -> *mut c_char {
    let _name = <Faker as PhoneNumber>::phone_number();
    let __name = _name.as_str();
    let full_name = CString::new(__name).unwrap();
    full_name.into_raw()
}



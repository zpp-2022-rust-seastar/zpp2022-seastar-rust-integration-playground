use std::ffi::{CString,CStr};
use std::os::raw::c_char; 

#[link(name = "rust_ffi")]
extern {
    fn store_from_rust(key: *const c_char, key: *const c_char);

    fn load_from_rust(key: *const c_char) -> *mut c_char;

    fn free_from_rust(key: *mut c_char);
}

#[no_mangle]
pub extern fn do_requests_from_rust() {
    let key = "rustkey";
    let value = "rustvalue";
    let key_c = CString::new(key).unwrap();
    let value_c = CString::new(value).unwrap();
    unsafe { 
        store_from_rust(key_c.as_ptr(), value_c.as_ptr());
        println!("DONE$");
        let loaded_value_ptr = load_from_rust(key_c.as_ptr());
        if loaded_value_ptr as usize == 0 { // loaded_value_ptr == NULL
            println!("NOTFOUND$");
        } else {
            let loaded_value = CStr::from_ptr(loaded_value_ptr).to_str().unwrap();
            println!("FOUND${loaded_value}$");
            free_from_rust(loaded_value_ptr);
        }
    }
}

#[link(name = "hello_from_cpp")]
extern {
    fn hello_from_cpp(x: std::ffi::c_int);
}

#[no_mangle]
pub extern fn hello_from_rust(x: i32) {
    println!("Hello from Rust with value {x}!");
    unsafe { hello_from_cpp(37); }
}

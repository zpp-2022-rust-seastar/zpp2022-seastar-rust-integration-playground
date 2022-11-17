#[link(name = "hello_from_cpp")]
extern {
    fn hello_from_cpp();
}

#[no_mangle]
pub extern fn hello_from_rust() {
    println!("Hello from Rust!");
    unsafe { hello_from_cpp(); }
}

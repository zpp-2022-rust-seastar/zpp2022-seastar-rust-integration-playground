use std::env;

fn main() {
    match env::var("HELLO_FROM_CPP_DIR") {
        Ok(dir) => println!(r"cargo:rustc-link-search={dir}"),
        Err(_) => panic!("HELLO_FROM_CPP_DIR not set.")
    }
}
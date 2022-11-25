use std::env;

fn main() {
    let build_dir;
    match env::var("BUILD_DIR") {
        Ok(dir) => build_dir = dir,
        Err(_) => panic!("BUILD_DIR not set.")
    }

    cxx_build::bridge("src/lib.rs")
        .file("src/cpp_part/line.cc")
        .flag_if_supported("-std=c++20")
        .object(format!("{build_dir}/libseastar_ffi.a"))
        .compile("cpp_ffi");
}
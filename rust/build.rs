use std::{fs, env};
use std::path::Path;
use std::io::Write;

fn copy_cxx_async_headers() {
    let build_path = env::current_dir().unwrap().parent().unwrap().join("build").join("rust");
    let dest_include_path = build_path.join("cxxbridge").join("rust");
    let deps_path = build_path.join("debug").join("build");

    let include_path = Path::new("out").join("include").join("rust");
    let cxx_async_dir = 
        fs::read_dir(deps_path)
        .unwrap()
        .flatten()
        .filter(|dir| dir.file_name().to_str().unwrap().starts_with("cxx-async"))
        .filter(|dir| dir.path().join(&include_path).exists())
        .next()
        .unwrap();
    let cxx_async_include_dir = 
        fs::read_dir(cxx_async_dir.path().join(include_path))
        .unwrap();

    for header in cxx_async_include_dir.flatten() {
        drop(fs::copy(header.path(), &dest_include_path.join(header.file_name())));
    }
}

fn main() {
    copy_cxx_async_headers();
    cxx_build::bridge("src/lib.rs")
        .file("../server/ffi.cc")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-fcoroutines")
        .define("SEASTAR_API_LEVEL", "6")
        .define("SEASTAR_SCHEDULING_GROUPS_COUNT", "16")
        .compile("cpp_ffi");
}

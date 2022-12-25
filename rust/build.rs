use std::{fs, env};
use std::path::Path;

fn copy_cxx_async_headers() {
    let common_path_prefix = env::current_dir().unwrap().parent().unwrap().join("build").join("rust");
    let dest_include_path = common_path_prefix.join("cxxbridge").join("rust");
    let cxx_async_path = common_path_prefix.join("debug").join("build");

    let include_path = Path::new("out").join("include").join("rust");
    let src_include_dir = fs::read_dir(cxx_async_path)
        .unwrap()
        .flatten()
        .filter(|dir| dir.file_name().to_str().unwrap().starts_with("cxx-async"))
        .filter(|dir| dir.path().join(&include_path).exists())
        .next()
        .unwrap();

    for header in fs::read_dir(src_include_dir.path()).unwrap().flatten() {
        drop(fs::copy(header.path(), &dest_include_path));
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

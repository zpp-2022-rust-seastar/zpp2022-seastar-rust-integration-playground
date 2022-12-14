fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("../server/ffi.cc")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-fcoroutines")
        .define("SEASTAR_API_LEVEL", "6")
        .define("SEASTAR_SCHEDULING_GROUPS_COUNT", "16")
        .include("../cxx-async/cxx-async/include")
        .compile("cpp_ffi");
}

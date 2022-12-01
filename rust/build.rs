fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++20")
        .define("SEASTAR_API_LEVEL", "6")
        .define("SEASTAR_SCHEDULING_GROUPS_COUNT", "16")
        .compile("cpp_ffi");
}
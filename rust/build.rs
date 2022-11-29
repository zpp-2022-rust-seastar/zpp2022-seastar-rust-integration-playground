fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-DSEASTAR_API_LEVEL=6")
        .flag_if_supported("-DSEASTAR_SCHEDULING_GROUPS_COUNT=16")
        .compile("cpp_ffi");
}
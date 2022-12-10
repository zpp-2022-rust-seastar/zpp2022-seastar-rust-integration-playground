fn main() {
    let tasks_dir = "../server/tasks";
    cxx_build::bridge("src/lib.rs")
        .file(format!("{tasks_dir}/store_task.cc"))
        .file(format!("{tasks_dir}/load_task.cc"))
        .flag_if_supported("-std=c++20")
        .define("SEASTAR_API_LEVEL", "6")
        .define("SEASTAR_SCHEDULING_GROUPS_COUNT", "16")
        .compile("cpp_ffi");
}

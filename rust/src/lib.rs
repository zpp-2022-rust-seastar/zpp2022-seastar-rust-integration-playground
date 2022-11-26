#[cxx::bridge(namespace = "rust")]
mod ffi {
    extern "Rust" {

    }

    unsafe extern "C++" {
        include!("rust/../server/tasks/store_task.hh");

    }
}

mod rust_storage;

use rust_storage::{create_rust_storage, load, store, RustStorage};

use std::future::Future;

#[cxx::bridge]
mod ffi {

    extern "Rust" {
        type RustStorage;
        fn create_rust_storage() -> Box<RustStorage>;
        fn store(storage: &'static mut RustStorage, key: &'static str, val: &'static str) -> StoreFuture;
        fn load(storage: &'static RustStorage, key: &'static str) -> LoadFuture;
    }

    unsafe extern "C++" {
        include!("rust/../server/ffi.hh");

        type StoreFuture = crate::StoreFuture;
        type LoadFuture = crate::LoadFuture;
        type SleepFuture = crate::SleepFuture;

        fn seastar_sleep_1s() -> SleepFuture;
    }
}

#[cxx_async::bridge]
unsafe impl Future for StoreFuture {
    type Output = ();
}

#[cxx_async::bridge]
unsafe impl Future for LoadFuture {
    type Output = String;
}

#[cxx_async::bridge]
unsafe impl Future for SleepFuture {
    type Output = ();
}
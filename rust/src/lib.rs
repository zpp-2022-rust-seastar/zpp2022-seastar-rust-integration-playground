mod rust_storage;

use rust_storage::{create_rust_storage, delete_rust_storage};
use rust_storage::RustStorage;
// use rust_storage::RustStorage::{hello, store, load};

mod store_future;

use store_future::{poll_store_future, create_store_future, delete_store_future};
use store_future::StoreFuture;

#[cxx::bridge(namespace = "rust")]
mod ffi {

    extern "Rust" {
        type RustStorage;
        // pub fn hello(storage: &RustStorage);
        // pub fn store(storage: &mut RustStorage, key: &str, val: &str);
        // pub fn load(storage: &RustStorage, key: &str) -> *const String;
        fn create_rust_storage() -> *mut RustStorage;
        unsafe fn delete_rust_storage(rs: *mut RustStorage);

        type StoreFuture;
        fn poll_store_future(task: Pin<&mut StoreTask>, out: &mut String) -> bool;
        unsafe fn create_store_future(storage: *mut RustStorage, key: String, value: String) -> *mut StoreFuture;
        unsafe fn delete_store_future(fut: *mut StoreFuture);
    }
    
    extern "C++" {
        type StoreTask;
    }

    unsafe extern "C++" {
        include!("rust/../server/tasks/store_task.hh");
    
        fn get_fut(self: Pin<&mut StoreTask>) -> &mut StoreFuture;

        fn wake_store_task(task: Pin<&mut StoreTask>);
    }
}


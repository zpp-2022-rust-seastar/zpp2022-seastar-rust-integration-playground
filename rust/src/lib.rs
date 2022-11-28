mod rust_storage;

use rust_storage::{create_rust_storage, delete_rust_storage};
use rust_storage::RustStorage;
// use rust_storage::RustStorage::{hello, store, load};

mod waker;

mod store_future;

use store_future::{poll_store_future, create_store_future, delete_store_future};
use store_future::StoreFuture;

mod load_future;

use load_future::{poll_load_future, create_load_future, delete_load_future};
use load_future::LoadFuture;

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

        type LoadFuture;
        fn poll_load_future(task: Pin<&mut LoadTask>, out: &mut String) -> bool;
        unsafe fn create_load_future(storage: *mut RustStorage, key: String) -> *mut LoadFuture;
        unsafe fn delete_load_future(fut: *mut LoadFuture);
    }
    
    extern "C++" {
        type StoreTask;
        type LoadTask;
    }

    unsafe extern "C++" {
        include!("rust/../server/tasks/store_task.hh");
        include!("rust/../server/tasks/load_task.hh");
    
        fn get_store_fut(self: Pin<&mut StoreTask>) -> &mut StoreFuture;

        fn wake_store_task(task: Pin<&mut StoreTask>);

        fn get_load_fut(self: Pin<&mut LoadTask>) -> &mut LoadFuture;

        fn wake_load_task(task: Pin<&mut LoadTask>);
    }
}


mod rust_storage;

use rust_storage::create_rust_storage;
use rust_storage::RustStorage;

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
        fn create_rust_storage() -> Box<RustStorage>;

        type StoreFuture<'a>;
        fn poll_store_future(task: Pin<&mut StoreTask>) -> bool;
        unsafe fn create_store_future(storage: &mut Box<RustStorage>, key: String, value: String) -> *mut StoreFuture;
        unsafe fn delete_store_future(fut: *mut StoreFuture);

        type LoadFuture<'a>;
        fn poll_load_future(task: Pin<&mut LoadTask>, out: &mut String) -> bool;
        unsafe fn create_load_future(storage: &mut Box<RustStorage>, key: String) -> *mut LoadFuture;
        unsafe fn delete_load_future(fut: *mut LoadFuture);

        fn not_found_constant() -> String;
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
        unsafe fn schedule_callback_for_store_future_after_one_second(cb: unsafe fn(*mut StoreFuture), data: *mut StoreFuture);

        fn get_load_fut(self: Pin<&mut LoadTask>) -> &mut LoadFuture;
        fn wake_load_task(task: Pin<&mut LoadTask>);
        unsafe fn schedule_callback_for_load_future_after_one_second(cb: unsafe fn(*mut LoadFuture), data: *mut LoadFuture);
    }
}

const NOT_FOUND: &str = "$NOT_FOUND";

fn not_found_constant() -> String {
    NOT_FOUND.to_string()
}

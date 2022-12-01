use std::future::Future;
use std::task::{Context, Poll, Waker, RawWaker};
use std::pin::Pin;

use crate::rust_storage::RustStorage;
use crate::waker::WAKER_VTABLE;

pub struct StoreFuture {
    running: bool,
    done: bool,
    waker: Option<Waker>,
    storage: *mut RustStorage,
    key: String,
    value: String,
}

impl Future for StoreFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            (*self.storage).store(&self.key[..], &self.value[..]);
        }
        Poll::Ready(())
    }
}

pub fn poll_store_future(task: Pin<&mut super::ffi::StoreTask>, _out: &mut String) -> bool {
    let waker = unsafe {
        Waker::from_raw(RawWaker::new(task.as_ref().get_ref() as *const super::ffi::StoreTask as *const (), &WAKER_VTABLE))
    };
    let fut = task.get_store_fut();
    let mut ctx = Context::from_waker(&waker);
    matches!(Pin::new(fut).poll(&mut ctx), Poll::Ready(_))
}

pub fn create_store_future(storage: *mut RustStorage, key: String, value: String) -> *mut StoreFuture {
    Box::into_raw(Box::new(StoreFuture {
        running: false,
        done: false,
        waker: None,
        storage: storage,
        key: key,
        value: value,
    }))
}

pub unsafe fn delete_store_future(fut: *mut StoreFuture) {
    let _ = Box::from_raw(fut);
}

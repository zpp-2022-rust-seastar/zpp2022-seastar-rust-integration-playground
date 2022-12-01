use std::future::Future;
use std::task::{Context, Poll, Waker, RawWaker};
use std::pin::Pin;

use crate::rust_storage::RustStorage;
use crate::waker::WAKER_VTABLE;

pub struct LoadFuture {
    running: bool,
    done: bool,
    waker: Option<Waker>,
    storage: *mut RustStorage,
    key: String,
}

impl Future for LoadFuture {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let value = 
        unsafe {
            (*self.storage).load(&self.key[..]).as_ref()
        };
        Poll::Ready(match value {
            None => String::new(), // TO FIX: this should be null if the value is not found
            Some(v) => String::from(v),
        })
    }
}

pub fn poll_load_future(task: Pin<&mut super::ffi::LoadTask>, out: &mut String) -> bool {
    let waker = unsafe {
        Waker::from_raw(RawWaker::new(task.as_ref().get_ref() as *const super::ffi::LoadTask as *const (), &WAKER_VTABLE))
    };
    let fut = task.get_load_fut();
    let mut ctx = Context::from_waker(&waker);
    match Pin::new(fut).poll(&mut ctx) {
        Poll::Pending => false,
        Poll::Ready(x) => {
            *out = x;
            true
        },
    }
}

pub fn create_load_future(storage: *mut RustStorage, key: String) -> *mut LoadFuture {
    Box::into_raw(Box::new(LoadFuture {
        running: false,
        done: false,
        waker: None,
        storage: storage,
        key: key,
    }))
}

pub unsafe fn delete_load_future(fut: *mut LoadFuture) {
    let _ = Box::from_raw(fut);
}

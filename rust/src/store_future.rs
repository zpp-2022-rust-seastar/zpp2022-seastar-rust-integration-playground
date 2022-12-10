use std::future::Future;
use std::task::{Context, Poll, Waker, RawWaker};
use std::pin::Pin;

use crate::ffi::schedule_callback_for_store_future_after_one_second;
use crate::rust_storage::RustStorage;
use crate::waker::STORE_WAKER_VTABLE;

pub struct StoreFuture<'a> {
    running: bool,
    done: bool,
    waker: Option<Waker>,
    storage: &'a mut Box<RustStorage>,
    key: String,
    value: String,
}

impl<'a> Future for StoreFuture<'a> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.running {
            self.as_mut().running = true;
            self.as_mut().waker = Some(ctx.waker().clone());
            fn callback(x: *mut StoreFuture) {
                unsafe {
                    (*x).done = true;
                    (*x).waker.take().map(|w| w.wake());
                }
            }
            unsafe {
                schedule_callback_for_store_future_after_one_second(callback, self.as_ref().get_ref() as *const StoreFuture as *mut StoreFuture);
            }
            return Poll::Pending;
        }

        if self.done {
            let k = &self.key.clone();
            let v = &self.value.clone();
            self.storage.store(k, v);
            println!("STORE${}${}$", self.key, self.value);
            Poll::Ready(())
        } else {
            Poll::Pending
        }        
    }
}

pub fn poll_store_future(task: Pin<&mut super::ffi::StoreTask>) -> bool {
    let waker = unsafe {
        Waker::from_raw(RawWaker::new(task.as_ref().get_ref() as *const super::ffi::StoreTask as *const (), &STORE_WAKER_VTABLE))
    };
    let fut = task.get_store_fut();
    let mut ctx = Context::from_waker(&waker);
    matches!(Pin::new(fut).poll(&mut ctx), Poll::Ready(_))
}

pub fn create_store_future(storage: &mut Box<RustStorage>, key: String, value: String) -> Box<StoreFuture> {
    Box::new(StoreFuture {
        running: false,
        done: false,
        waker: None,
        storage,
        key,
        value,
    })
}

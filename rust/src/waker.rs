use std::task::{RawWaker, RawWakerVTable};
use std::pin::Pin;

pub static WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake,
    waker_drop,
);

fn waker_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &WAKER_VTABLE)
}

unsafe fn waker_wake(data :*const ()) {
    super::ffi::wake_store_task(Pin::new_unchecked(&mut *(data as *const super::ffi::StoreTask as *mut super::ffi::StoreTask)));
}

fn waker_drop(_data: *const ()) {

}
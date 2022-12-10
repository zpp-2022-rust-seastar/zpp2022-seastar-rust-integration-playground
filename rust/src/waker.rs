use std::task::{RawWaker, RawWakerVTable};
use std::pin::Pin;

pub static STORE_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    store_waker_clone,
    store_waker_wake,
    store_waker_wake,
    waker_drop,
);

pub static LOAD_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    load_waker_clone,
    load_waker_wake,
    load_waker_wake,
    waker_drop,
);

fn store_waker_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &STORE_WAKER_VTABLE)
}

fn load_waker_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &LOAD_WAKER_VTABLE)
}

unsafe fn store_waker_wake(data :*const ()) {
    super::ffi::wake_store_task(Pin::new_unchecked(&mut *(data as *const super::ffi::StoreTask as *mut super::ffi::StoreTask)));
}

unsafe fn load_waker_wake(data :*const ()) {
    super::ffi::wake_load_task(Pin::new_unchecked(&mut *(data as *const super::ffi::LoadTask as *mut super::ffi::LoadTask)));
}


fn waker_drop(_data: *const ()) {

}
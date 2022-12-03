#include "store_task.hh"
#include "rust/src/lib.rs.h"
#include "seastar/core/sleep.hh"

namespace rust {

void StoreTask::schedule_me() {
    if (!_scheduled) {
        seastar::schedule(this);
        _scheduled = true;
    }
}

void StoreTask::run_and_dispose() noexcept {
    _scheduled = false;
    if (poll_store_future(*this)) {
        this->_pr.set_value();
        delete this;
    }
}

StoreFuture& StoreTask::get_store_fut() {
    return *_rfut;
}

StoreTask::StoreTask(RustStorage* rust_storage, const std::string& key, const std::string& val) : continuation_base_with_promise(seastar::promise<>()) {
    printf("Here I am: %p\n", this);
    _rfut = create_store_future(rust_storage, String(key), String(val));
}

StoreTask::~StoreTask() {
    delete_store_future(_rfut);
}

seastar::future<> StoreTask::get_future() {
    return _pr.get_future();
}

void wake_store_task(StoreTask& task) {
    printf("Task: %p\n", &task);
    task.schedule_me();
}

void schedule_callback_for_store_future_after_one_second(Fn<void(StoreFuture*)> fn, StoreFuture* data) {
    (void)seastar::sleep(std::chrono::seconds(1)).then([fn, data] {
        fn(data);
    });
}

} // namespace rust

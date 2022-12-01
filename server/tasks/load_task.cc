#include "load_task.hh"
#include "rust/src/lib.rs.h"
#include "seastar/core/sleep.hh"

namespace rust {

void LoadTask::schedule_me() {
    if (!_scheduled) {
        seastar::schedule(this);
        _scheduled = true;
    }
}

void LoadTask::run_and_dispose() noexcept {
    _scheduled = false;
    rust::String out;
    if (rust::poll_load_future(*this, out)) {
        this->_pr.set_value(out);
        delete this;
    }
}

LoadFuture& LoadTask::get_load_fut() {
    return *_rfut;
}

LoadTask::LoadTask(RustStorage* rust_storage, std::string& key) : continuation_base_with_promise(seastar::promise<std::string>()) {
    printf("Here I am: %p\n", this);
    _rfut = rust::create_load_future(rust_storage, key);
}

LoadTask::~LoadTask() {
    rust::delete_load_future(_rfut);
}

seastar::future<std::string> LoadTask::get_future() {
    return _pr.get_future();
}

void wake_rust_task(LoadTask& task) {
    printf("Task: %p\n", &task);
    task.schedule_me();
}

void schedule_callback_after_one_second(rust::Fn<void(LoadFuture*)> fn, LoadFuture* data) {
    (void)seastar::sleep(std::chrono::seconds(1)).then([fn, data] {
        fn(data);
    });
}

} // namespace rust

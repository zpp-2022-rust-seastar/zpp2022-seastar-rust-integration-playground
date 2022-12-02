#pragma once

#include <seastar/core/future.hh>
#include "rust/cxx.h"

namespace rust {

struct LoadFuture;
struct RustStorage;

struct LoadTask: public seastar::continuation_base_with_promise<seastar::promise<std::string>, std::string> {
    LoadFuture* _rfut;
    bool _scheduled = true;

    void schedule_me();

    virtual void run_and_dispose() noexcept;

    LoadFuture& get_load_fut();

    LoadTask(RustStorage* rust_storage, const std::string& key);

    virtual ~LoadTask();

    seastar::future<std::string> get_future();
};

void wake_load_task(LoadTask& task);

void schedule_callback_after_one_second(rust::Fn<void(LoadFuture*)> fn, LoadFuture* data);

} // namespace rust

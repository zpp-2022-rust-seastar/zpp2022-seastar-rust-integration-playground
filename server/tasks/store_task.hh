#pragma once

#include <seastar/core/future.hh>
#include "rust/cxx.h"

namespace rust {

struct StoreFuture;
struct RustStorage;

struct StoreTask : public seastar::continuation_base_with_promise<seastar::promise<>, void> {
    StoreFuture* _rfut;
    bool _scheduled = true;

    void schedule_me();

    virtual void run_and_dispose() noexcept;

    StoreFuture& get_store_fut();

    StoreTask(RustStorage* rust_storage, std::string& key, std::string& val);

    virtual ~StoreTask();

    seastar::future<> get_future();
};

void wake_store_task(StoreTask& task);

void schedule_callback_after_one_second(rust::Fn<void(StoreFuture*)> fn, StoreFuture* data);

} // namespace rust

#pragma once

#include "rust/cxx.h"

namespace rust {
    struct StoreFuture;

    struct StoreTask {
        rust::StoreFuture* _rfut;
        bool _scheduled = true;

        void schedule_me();

        virtual void run_and_dispose() noexcept;

        StoreFuture& get_fut();

        StoreTask();

        virtual ~StoreTask();

        //seastar::future<uint32_t> get_future();
    };

    void wake_store_task(StoreTask& task);

    void schedule_callback_after_one_second(rust::Fn<void(StoreFuture*)> fn, StoreFuture* data);
}

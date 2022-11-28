#pragma once

#include "rust/cxx.h"

namespace rust {
    struct LoadFuture;

    struct LoadTask {
        rust::LoadFuture* _rfut;
        bool _scheduled = true;

        void schedule_me();

        virtual void run_and_dispose() noexcept;

        LoadFuture& get_load_fut();

        LoadTask();

        virtual ~LoadTask();

        //seastar::future<uint32_t> get_future();
    };

    void wake_load_task(LoadTask& task);

    void schedule_callback_after_one_second(rust::Fn<void(LoadFuture*)> fn, LoadFuture* data);
}

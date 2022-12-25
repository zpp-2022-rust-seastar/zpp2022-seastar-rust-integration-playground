#pragma once

#include "rust/cxx.h"
#include "rust/cxx_async.h"

CXXASYNC_DEFINE_FUTURE(void, StoreFuture);
CXXASYNC_DEFINE_FUTURE(rust::String, LoadFuture);
CXXASYNC_DEFINE_FUTURE(void, SleepFuture);

SleepFuture seastar_sleep_1s();
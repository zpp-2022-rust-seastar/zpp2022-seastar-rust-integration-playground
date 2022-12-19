#include "ffi.hh"
#include <seastar/core/sleep.hh>

SleepFuture seastar_sleep_1s() {
    co_return co_await seastar::sleep(std::chrono::seconds(1));
}
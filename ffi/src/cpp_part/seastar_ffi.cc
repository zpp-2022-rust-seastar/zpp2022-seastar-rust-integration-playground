#include <seastar/core/app-template.hh>
#include <iostream>

#include "seastar_ffi.h"

void do_something_using_seastar() {
    auto xd = seastar::make_ready_future<>();
}

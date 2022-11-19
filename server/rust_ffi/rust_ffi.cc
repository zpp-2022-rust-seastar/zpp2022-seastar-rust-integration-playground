#include <iostream>
#include "rust_ffi.h"

extern "C" void hello_from_cpp(int x) {
    std::cout << "Hello from C++ with value " << x << "!\n";
}
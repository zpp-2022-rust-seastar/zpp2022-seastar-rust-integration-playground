#include <iostream>
#include "hello_from_cpp.h"

extern "C" void hello_from_cpp() {
    std::cout << "Hello from C++!\n";
}
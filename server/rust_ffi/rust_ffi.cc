#include <iostream>
#include <cstring>
#include "rust_ffi.h"

extern "C" void store_from_rust(const char *key, const char *value) {
    std::cout << "STORE$" << key << "$" << value << "$\n";
}

// Returns pointer to loaded value. If value is not found, returns NULL pointer.
extern "C" char * load_from_rust(const char *key) {
    std::cout << "LOAD$" << key << "$\n";

    const char *value = "rustvalue";
    return strdup(value);
}

extern "C" void free_from_rust(char *ptr) {
    free(ptr);
}

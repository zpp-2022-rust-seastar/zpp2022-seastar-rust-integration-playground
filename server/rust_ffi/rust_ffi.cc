#include <iostream>
#include <cstring>

#include "rust_ffi.h"
#include "../server.hh"

using namespace seastar;

extern "C" void store_from_rust(const char *key, const char *value) {
    std::cout << "STORE$" << key << "$" << value << "$\n";

    tcp_server::rust_data[std::string(key)] = std::string(value);
}

// Returns pointer to loaded value. If value is not found, returns NULL pointer.
extern "C" char * load_from_rust(const char *key) {
    std::cout << "LOAD$" << key << "$\n";

    if (!tcp_server::rust_data.contains(std::string(key))) {
        return NULL;
    }

    std::string value = tcp_server::rust_data[std::string(key)];
    return strdup(value.c_str());
}

extern "C" void free_from_rust(char *ptr) {
    free(ptr);
}

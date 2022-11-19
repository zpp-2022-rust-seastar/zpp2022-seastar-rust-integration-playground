project(TCP-Server)

cmake_minimum_required(VERSION 3.5)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "-fcoroutines")

find_package (Valgrind)
find_package (Seastar REQUIRED)

# Build interface used by Rust.
add_library(rust_ffi STATIC server/rust_ffi/rust_ffi.cc)
target_compile_options(rust_ffi PRIVATE "-fPIC")

# Build Rust module.
add_custom_target(
    rust_requests
    COMMAND cd ${CMAKE_SOURCE_DIR}/rust_requests &&
            RUST_FFI_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo build
            --release --target-dir ${CMAKE_SOURCE_DIR}/build/rust-target
    DEPENDS rust_ffi
)

# Build server.
add_executable(server
    server/main.cc
    server/server.cc
)
add_dependencies(server rust_requests)
target_link_libraries(server
    Seastar::seastar
    ${CMAKE_SOURCE_DIR}/build/rust-target/release/librust_requests.so
)

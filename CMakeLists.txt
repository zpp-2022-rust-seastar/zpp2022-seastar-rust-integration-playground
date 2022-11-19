project(TCP-Server)

cmake_minimum_required(VERSION 3.5)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "-fcoroutines")

find_package (Valgrind)
find_package (Seastar REQUIRED)

# Build interface used by Rust.
add_library(hello_from_cpp STATIC cpp/hello_from_cpp.cc)
target_compile_options(hello_from_cpp PRIVATE "-fPIC")

# Build Rust module.
add_custom_target(
    hello_from_rust
    COMMAND cd ${CMAKE_SOURCE_DIR}/rust/ &&
            HELLO_FROM_CPP_DIR=${CMAKE_CURRENT_BINARY_DIR}
            cargo build --release --target-dir ${CMAKE_SOURCE_DIR}/build/rust-target
    DEPENDS hello_from_cpp
)

# Build server.
add_executable(server
    cpp/main.cc
    cpp/server.cc
)
add_dependencies(server hello_from_rust)
target_link_libraries(server
    Seastar::seastar
    ${CMAKE_SOURCE_DIR}/build/rust-target/release/libhello_from_rust.so
)

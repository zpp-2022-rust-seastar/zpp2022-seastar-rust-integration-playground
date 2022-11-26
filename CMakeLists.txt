project(TCP-Server)

cmake_minimum_required(VERSION 3.5)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "-fcoroutines")

find_package(Valgrind)
find_package(Seastar REQUIRED)

set(RUST_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR}/rust)

# Build Rust part.
add_custom_target(
    rust
    COMMAND cd ${CMAKE_SOURCE_DIR}/rust &&
            cargo build --target-dir ${RUST_TARGET_DIR}
)

# Build server.
add_executable(
    server
    server/main.cc
    server/server.cc
    server/tasks/line.cc
)
add_dependencies(server rust)
target_link_libraries(
    server
    Seastar::seastar
    ${RUST_TARGET_DIR}/debug/librust.a
)
target_include_directories(
    server
    PRIVATE ${RUST_TARGET_DIR}/cxxbridge
    PRIVATE ${CMAKE_SOURCE_DIR}
)

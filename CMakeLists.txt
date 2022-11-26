project(TCP-Server)

cmake_minimum_required(VERSION 3.5)

set(CMAKE_CXX_STANDARD 20)

find_package(Valgrind)
find_package(Seastar REQUIRED)

set(RUST_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR}/ffi)

# Build ffi.
add_custom_target(
    ffi
    COMMAND cd ${CMAKE_SOURCE_DIR}/ffi &&
            cargo build --target-dir ${RUST_TARGET_DIR}
)

# Build server.
add_executable(
    server
    ffi/src/cpp_part/line.cc
    server/main.cc
    server/server.cc
)
add_dependencies(server ffi)
target_link_libraries(
    server
    Seastar::seastar
    ${RUST_TARGET_DIR}/debug/libffi.a
)
target_include_directories(
    server
    PRIVATE ${RUST_TARGET_DIR}/cxxbridge
    PRIVATE ${CMAKE_SOURCE_DIR}
)

project(Hello)

cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 20)

# We create a hello_from_cpp library that will be used by Rust.
# This is a static library (.a file).
# Note that hello_from_cpp function is extern "C".
add_library(hello_from_cpp STATIC cpp/hello_from_cpp.cc)
# Adding -fPIC flag is necessery. Read this
# https://stackoverflow.com/questions/5311515/gcc-fpic-option
target_compile_options(hello_from_cpp PRIVATE "-fPIC")

# This is the trickiest part. We create a custom target repsponsible
# for compiling a Rust library - hello_from_rust.
# "COMMAND" is the script executed to create target hello_from_rust
# (or, in other words, to compile our Rust library).
# "cd ${CMAKE_SOURCE_DIR}/rust/" goes into rust directory.
# "HELLO_FROM_CPP_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo build"
# compiles Rust library with environment variable 
# CMAKE_CURRENT_BINARY_DIR set to the direct path to build directory
# (check rust/build.rs). So this works regardless of the build
# directory location.
# "DEPENDS hello_from_cpp" ensures that hello_from_rust is build
# after hello_from_cpp which is necessery because hello_from_rust
# uses hello_from_cpp.
add_custom_target(
    hello_from_rust
    COMMAND cd ${CMAKE_SOURCE_DIR}/rust/ &&
            HELLO_FROM_CPP_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo build --release
    DEPENDS hello_from_cpp
)

add_executable(main cpp/main.cc)
# We ensure that main is build after hello_from_rust.
add_dependencies(main hello_from_rust)
# We link hello_from_rust to main.
target_link_libraries(
    main 
    ${CMAKE_SOURCE_DIR}/rust/target/release/libhello_from_rust.so
)

project(Hello)

cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 20)

add_library(hello_from_cpp STATIC cpp/hello_from_cpp.cc)
target_compile_options(hello_from_cpp PRIVATE "-fPIC")

add_custom_target(
    hello_from_rust
    COMMAND cd ${CMAKE_SOURCE_DIR}/rust/ &&
            HELLO_FROM_CPP_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo build
    DEPENDS hello_from_cpp
)

add_executable(main cpp/main.cc)
add_dependencies(main hello_from_rust)
target_link_libraries(
    main 
    ${CMAKE_SOURCE_DIR}/rust/target/debug/libhello_from_rust.so
)

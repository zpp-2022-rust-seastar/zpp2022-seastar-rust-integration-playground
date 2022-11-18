# zpp2022-seastar-rust-integration-playground

## Hello example

### What is going on here?
There are 3 parts of the program:
- Function `main` (which is in C++) calls Rust `hello_from_rust` function.
- Function `hello_from_rust` prints something and calls C++ function `hello_from_cpp`.
- Function `hello_from_cpp` prints something.

So we go from C++ to Rust and then back to C++. We have 2 dependencies here:
- `hello_from_rust` depends on `hello_from_cpp`,
- `main` depends on `hello_from_rust`.

These dependencies force us to build parts of our example in the following way:
- `hello_from_cpp`
- `hello_from_rust`
- `main`

Library `hello_from_cpp` is static (`.a`) and `hello_from_rust` is dynamic (`.so`). Why? Because currently this is the only way I can do it. There are probably many other ways to make it work.

### Recommended use

Clone this repository and go to its directory. Then execute
```
mkdir build
cd build
cmake \
-DCMAKE_PREFIX_PATH="~/repos/seastar/build/release;~/repos/seastar/build/release/_cooking/installed" \
-DCMAKE_MODULE_PATH=~/repos/seastar/cmake \
..
make
./main
```

### Manual compilation (for educational purposes)

Create a C++ static library `hello_from_cpp` that will be used by Rust.
```
cd cpp
g++ -c -fPIC hello_from_cpp.cc -o hello_from_cpp.o
ar rcs libhello_from_cpp.a hello_from_cpp.o
```
Create a Rust dynamic library `hello_from_rust` that uses `hello_from_cpp` and will be used by `main`.
```
cd ../rust
HELLO_FROM_CPP_DIR="$(pwd)/../cpp" cargo build --release
```
Compile and execute `main` that uses `hello_from_rust`.
```
cd ../cpp
g++ main.cc -o main -lhello_from_rust -L../rust/target/release
LD_LIBRARY_PATH=../rust/target/release ./main
```

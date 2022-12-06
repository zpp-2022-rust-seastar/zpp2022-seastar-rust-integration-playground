# zpp2022-seastar-rust-integration-playground

### What is going on here?
- Function `main` (from `server/main.cc`) calls Rust `do_requests_from_rust` function.
- Function `do_requests_from_rust` performs some STORE/LOAD reqeusts. It uses functions from `server/rust_ffi/rust_ffi.h`.
- Functions `store_from_rust`, `load_from_rust` (from `server/rust_ffi/rust_ffi.h`) use static hash map `tcp_server::rust_data`.

We create 3 targets (appearing in `CMakeLists.txt`):
- `rust_ffi` - interface used by Rust module `rust_requests`,
- `rust_requests` - Rust module used by `server` that performs some requests,
- `server` - executable asking `rust_requests` to perform reqeusts and running main functionality of the server.

### Recommended use

Clone this repository and go to its directory. Then execute
```
mkdir build
cd build
cmake ..
make
./main
```
If it does not work, this might help
```
SEASTAR_PATH=<whatever this is for you>
mkdir build
cd build
cmake \
-DCMAKE_PREFIX_PATH="${SEASTAR_PATH}/build/release;${SEASTAR_PATH}/build/release/_cooking/installed" \
-DCMAKE_MODULE_PATH="${SEASTAR_PATH}/cmake" \
..
make
./main
```

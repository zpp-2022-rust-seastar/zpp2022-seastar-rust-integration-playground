# zpp2022-seastar-rust-integration-playground

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

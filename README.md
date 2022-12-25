# zpp2022-seastar-rust-integration-playground

## Usage
```
SEASTAR_PATH=...
```

### If Seastar is not installed:
```
cmake -B ./build \
-DCMAKE_MODULE_PATH="${SEASTAR_PATH}/cmake" \
-DCMAKE_PREFIX_PATH="${SEASTAR_PATH}/build/release;${SEASTAR_PATH}/build/release/_cooking/installed"
cmake --build ./build
./build/server
```
    
### If Seastar is installed:
```
cmake -B ./build \
-DCMAKE_MODULE_PATH="${SEASTAR_PATH}/cmake"
cmake --build ./build
./build/server
```
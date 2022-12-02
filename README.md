# zpp2022-seastar-rust-integration-playground

## Use
```
SEASTAR_PATH=...
cmake -B ./build \
-DCMAKE_MODULE_PATH="${SEASTAR_PATH}/cmake" \
-DCMAKE_PREFIX_PATH="${SEASTAR_PATH}/build/release;${SEASTAR_PATH}/build/release/_cooking/installed"
cmake --build ./build
./build/server

```

`DCMAKE_PREFIX_PATH` is not needed if you have seastar installed.
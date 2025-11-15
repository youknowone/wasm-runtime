# Simple WASM Runtime

WebAssembly runtime POC with wasmer with HashMap-based KV store.
First make sure to install wat2wasm and rust.

```bash
# following command installs rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build rustpython_wasm crate
pushd rustpython_wasm; cargo build; popd

wat2wasm wasm/test.wat -o wasm/test.wasm
cargo run --release wasm/test.wasm rustpython_wasm/target/wasm32-unknown-unknown/debug/rustpython_wasm.wasm return_eval

# For testing with wasm/module.wat having eval(i32, i32) -> i32 function
wat2wasm wasm/module.wat -o wasm/module.wasm
cargo run --release wasm/test.wasm wasm/module.wasm return_eval
```

## WASM binary requirements

Entry point is `process() -> i32`, following are exported functions, on 
error return -1:

- `kv_put(key_ptr: i32, key_len: i32, val_ptr: i32, val_len: i32) -> i32`
- `kv_get(key_ptr: i32, key_len: i32, val_ptr: i32, val_len: i32) -> i32`

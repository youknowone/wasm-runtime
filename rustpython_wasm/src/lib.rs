// From https://github.com/RustPython/RustPython/tree/main/wasm/wasm-unknown-test
use rustpython_vm::{Interpreter, VirtualMachine, pymodule};

#[pymodule]
mod new_module {
    use super::*;
    use rustpython_vm::builtins::PyBytesRef;

    #[pyfunction]
    fn py_hash(data: PyBytesRef, iterations: usize) -> u64 {
        let data = data.as_bytes();
        let mut hash = 0xcbf29ce484222325u64;
        for _ in 0..iterations {
            for &byte in data {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        hash
    }
}

// Host functions provided by the WASM runtime (main.rs)
unsafe extern "C" {
    /// kp and kl are the key pointer and length in wasm memory, vp and vl are for the return value
    /// if read value is bigger than vl then it will be truncated to vl, returns read bytes
    fn kv_get(kp: i32, kl: i32, vp: i32, vl: i32) -> i32;

    /// kp and kl are the key pointer and length in wasm memory, vp and vl are for the value
    fn kv_put(kp: i32, kl: i32, vp: i32, vl: i32) -> i32;
}

fn with_interpreter<F, R>(f: F) -> R
where
    F: FnOnce(&Interpreter) -> R,
{
    thread_local! {
        static INTERPRETER: Interpreter = {
            let mut interp = Interpreter::with_init(Default::default(), |vm| {
                vm.add_native_module("new_module", Box::new(new_module::make_module));
            });
            interp
        }
    }

    INTERPRETER.with(|interp| f(interp))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare() -> i32 {
    with_interpreter(|_| {});
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process() -> i32 {
    // 1. Read Python code from kv store
    let code_key = b"code";
    let mut code_buffer = vec![0u8; 4096]; // Buffer for code (max 4KB)

    let code_len = unsafe {
        kv_get(
            code_key.as_ptr() as i32,
            code_key.len() as i32,
            code_buffer.as_mut_ptr() as i32,
            code_buffer.len() as i32,
        )
    };

    if code_len <= 0 {
        return -2; // Failed to read code
    }

    code_buffer.truncate(code_len as usize);
    let src = match std::str::from_utf8(&code_buffer) {
        Ok(s) => s,
        Err(_) => return -2, // Invalid UTF-8
    };

    // 2. Execute Python code
    let result = with_interpreter(|interpreter| {
        interpreter.enter(|vm| {
            let scope = vm.new_scope_with_builtins();
            let res = match vm.run_block_expr(scope, src) {
                Ok(val) => val,
                Err(_) => return Err(-1), // Python execution error
            };
            let repr_str = match res.repr(vm) {
                Ok(repr) => repr.as_str().to_string(),
                Err(_) => return Err(-1), // Failed to get string representation
            };
            Ok(repr_str)
        })
    });
    let result = match result {
        Ok(r) => r,
        Err(code) => return code,
    };

    // 3. Store result in kv store
    let result_key = b"result";
    let result_bytes = result.into_bytes();

    let store_status = unsafe {
        kv_put(
            result_key.as_ptr() as i32,
            result_key.len() as i32,
            result_bytes.as_ptr() as i32,
            result_bytes.len() as i32,
        )
    };

    if store_status < 0 {
        return -2; // Failed to store result
    }

    0
}

#[unsafe(no_mangle)]
unsafe extern "Rust" fn __getrandom_v03_custom(
    _dest: *mut u8,
    _len: usize,
) -> Result<(), getrandom::Error> {
    // FIXME: Correct implementation
    return Ok(());
    // Err(getrandom::Error::UNSUPPORTED)
}

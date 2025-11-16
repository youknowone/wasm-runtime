// From https://github.com/RustPython/RustPython/tree/main/wasm/wasm-unknown-test
use rustpython_vm::Interpreter;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process() -> i32 {
    let src = "22 + 33";
    let interpreter = Interpreter::without_stdlib(Default::default());
    return interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        let res = match vm.run_block_expr(scope, src) {
            Ok(val) => val,
            Err(_) => return -1,
        };
        res.try_into_value(vm).unwrap_or(3000) // 3000 means just error code for testing.
    });
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

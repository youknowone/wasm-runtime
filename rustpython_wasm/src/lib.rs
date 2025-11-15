// From https://github.com/RustPython/RustPython/tree/main/wasm/wasm-unknown-test
use rustpython_vm::{Interpreter, eval};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn eval(s: *const u8, l: usize) -> u32 {
    let src = std::slice::from_raw_parts(s, l);
    let src = std::str::from_utf8(src).unwrap();
    let interpreter = Interpreter::without_stdlib(Default::default());
    return interpreter.enter(|vm| {
        let res = eval::eval(vm, "2+10000", vm.new_scope_with_builtins(), "<string>").unwrap();
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

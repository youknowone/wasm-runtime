use std::collections::HashMap;
use std::time::Instant;
use wasmer::{
    Function, FunctionEnv, FunctionEnvMut, Instance, Memory, Module, Store, Value, imports,
};

struct Ctx {
    kv: HashMap<Vec<u8>, Vec<u8>>,
    mem: Option<Memory>,
}

/// kp and kl are the key pointer and length in wasm memory, vp and vl are for the return value
/// if read value is bigger than vl then it will be truncated to vl, returns read bytes
fn kv_get(mut ctx: FunctionEnvMut<Ctx>, kp: i32, kl: i32, vp: i32, vl: i32) -> i32 {
    let (c, s) = ctx.data_and_store_mut();
    let mut key = vec![0u8; kl as usize];
    if c.mem
        .as_ref()
        .unwrap()
        .view(&s)
        .read(kp as u64, &mut key)
        .is_err()
    {
        return -1;
    }
    match c.kv.get(&key) {
        Some(val) => {
            let len = val.len().min(vl as usize);
            if c.mem
                .as_ref()
                .unwrap()
                .view(&s)
                .write(vp as u64, &val[..len])
                .is_err()
            {
                return -1;
            }
            len as i32
        }
        None => 0,
    }
}

/// kp and kl are the key pointer and length in wasm memory, vp and vl are for the value
fn kv_put(mut ctx: FunctionEnvMut<Ctx>, kp: i32, kl: i32, vp: i32, vl: i32) -> i32 {
    let (c, s) = ctx.data_and_store_mut();
    let mut key = vec![0u8; kl as usize];
    let mut val = vec![0u8; vl as usize];
    let m = c.mem.as_ref().unwrap().view(&s);
    if m.read(kp as u64, &mut key).is_err() || m.read(vp as u64, &mut val).is_err() {
        return -1;
    }
    c.kv.insert(key, val);
    0
}

fn rust_benchmark(iterations: usize, data: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for _ in 0..iterations {
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}

fn main() {
    let mut store = Store::default();
    let module = Module::new(
        &store,
        &std::fs::read(&std::env::args().nth(1).unwrap()).unwrap(),
    )
    .unwrap();

    // Prepare initial KV store with Python benchmark code
    let mut initial_kv = HashMap::new();
    let python_code = r#"
def py_hash(data, iterations):
    h = 0xcbf29ce484222325
    for _ in range(iterations):
        for byte in data:
            h ^= byte
            h = (h * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return h

# run 10 000 iterations
data = bytes(range(256))
result = py_hash(data, 10000)
str(result)
"#;
    initial_kv.insert(b"code".to_vec(), python_code.as_bytes().to_vec());

    let env = FunctionEnv::new(
        &mut store,
        Ctx {
            kv: initial_kv,
            mem: None,
        },
    );
    let imports = imports! {
        "env" => {
            "kv_get" => Function::new_typed_with_env(&mut store, &env, kv_get),
            "kv_put" => Function::new_typed_with_env(&mut store, &env, kv_put),
        }
    };
    let inst = Instance::new(&mut store, &module, &imports).unwrap();
    env.as_mut(&mut store).mem = inst.exports.get_memory("memory").ok().cloned();

    let start_python = Instant::now();
    let res = inst
        .exports
        .get_function("process")
        .unwrap()
        .call(&mut store, &[])
        .unwrap();
    let python_duration = start_python.elapsed();

    let python_result = env
        .as_ref(&store)
        .kv
        .get(&b"result".to_vec())
        .expect("No result found");

    println!("result code: {}", match res[0] {
        Value::I32(v) => v,
        _ => -1,
    });
    let python_result = std::str::from_utf8(python_result).unwrap();
    println!("Python execution time: {:?} {python_result}", python_duration);

    // Run native Rust benchmark
    let data: Vec<u8> = (0..=255).collect();
    let start_rust = Instant::now();
    let rust_result = rust_benchmark(10_000, &data); // 10k iterations
    let rust_duration = start_rust.elapsed();

    println!("Rust execution time: {:?} {rust_result}", rust_duration);
}

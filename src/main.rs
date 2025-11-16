use std::collections::HashMap;
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

fn main() {
    let mut store = Store::default();
    let module = Module::new(
        &store,
        &std::fs::read(&std::env::args().nth(1).unwrap()).unwrap(),
    )
    .unwrap();
    let env = FunctionEnv::new(
        &mut store,
        Ctx {
            kv: HashMap::new(),
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
    let res = inst
        .exports
        .get_function("process")
        .unwrap()
        .call(&mut store, &[])
        .unwrap();
    println!(
        "Result: {}",
        match res[0] {
            Value::I32(v) => v,
            _ => -1,
        }
    );
    println!("HashMap: {:?}", env.as_ref(&store).kv);
}

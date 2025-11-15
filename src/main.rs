use std::collections::HashMap;
use wasmer::{
    Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Memory, Module, Store, Value, imports,
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

    let rustpython_module_path = &std::env::args().nth(2).unwrap();
    let function_name = &std::env::args().nth(3).unwrap_or("process".to_string());

    println!("Start to load RustPython module");
    let rustpython_module = Module::from_file(&store, rustpython_module_path).unwrap();
    println!("Loaded RustPython module");
    let rustpython_inst = Instance::new(&mut store, &rustpython_module, &Imports::new()).unwrap();
    println!("Loaded RustPython instance module");
    let rustpython_eval = rustpython_inst
        .exports
        .get_function("eval")
        .unwrap()
        .clone();
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
        },
        "rustpython" => {
            "eval" => rustpython_eval,
        }
    };
    let inst = Instance::new(&mut store, &module, &imports).unwrap();
    env.as_mut(&mut store).mem = inst.exports.get_memory("memory").ok().cloned();
    let res = match inst
        .exports
        .get_function(function_name)
        .unwrap()
        .call(&mut store, &[])
    {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    println!(
        "Result: {}",
        match res[0] {
            Value::I32(v) => v,
            _ => -1,
        }
    );
    println!("HashMap: {:?}", env.as_ref(&store).kv);
}

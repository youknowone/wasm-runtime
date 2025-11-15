(module
  (import "env" "kv_put" (func $kv_put (param i32 i32 i32 i32) (result i32)))
  (import "env" "kv_get" (func $kv_get (param i32 i32 i32 i32) (result i32)))
  (import "rustpython" "eval" (func $eval (param i32 i32) (result i32)))

  (memory (export "memory") 1)

  (data (i32.const 50000) "1")

  (func (export "process") (result i32)
    (local $sum i32)
    (local $len i32)
    (local $i i32)

    i32.const 0
    i32.const 1
    call $eval
    drop

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 49))
    (i32.store8 (i32.const 10) (i32.const 10))
    (i32.store8 (i32.const 11) (i32.const 20))
    (call $kv_put (i32.const 0) (i32.const 2) (i32.const 10) (i32.const 2))
    drop

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 50))
    (i32.store8 (i32.const 10) (i32.const 30))
    (call $kv_put (i32.const 0) (i32.const 2) (i32.const 10) (i32.const 1))
    drop

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 51))
    (i32.store8 (i32.const 10) (i32.const 40))
    (i32.store8 (i32.const 11) (i32.const 50))
    (i32.store8 (i32.const 12) (i32.const 60))
    (call $kv_put (i32.const 0) (i32.const 2) (i32.const 10) (i32.const 3))
    drop

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 52))
    (i32.store8 (i32.const 10) (i32.const 70))
    (i32.store8 (i32.const 11) (i32.const 80))
    (i32.store8 (i32.const 12) (i32.const 90))
    (i32.store8 (i32.const 13) (i32.const 100))
    (call $kv_put (i32.const 0) (i32.const 2) (i32.const 10) (i32.const 4))
    drop

    (local.set $sum (i32.const 0))

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 49))
    (local.set $len (call $kv_get (i32.const 0) (i32.const 2) (i32.const 100) (i32.const 10)))
    (local.set $i (i32.const 0))
    (block $break1 (loop $loop1
      (br_if $break1 (i32.ge_s (local.get $i) (local.get $len)))
      (local.set $sum (i32.add (local.get $sum) (i32.load8_u (i32.add (i32.const 100) (local.get $i)))))
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (br $loop1)
    ))

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 50))
    (local.set $len (call $kv_get (i32.const 0) (i32.const 2) (i32.const 100) (i32.const 10)))
    (local.set $i (i32.const 0))
    (block $break2 (loop $loop2
      (br_if $break2 (i32.ge_s (local.get $i) (local.get $len)))
      (local.set $sum (i32.add (local.get $sum) (i32.load8_u (i32.add (i32.const 100) (local.get $i)))))
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (br $loop2)
    ))

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 51))
    (local.set $len (call $kv_get (i32.const 0) (i32.const 2) (i32.const 100) (i32.const 10)))
    (local.set $i (i32.const 0))
    (block $break3 (loop $loop3
      (br_if $break3 (i32.ge_s (local.get $i) (local.get $len)))
      (local.set $sum (i32.add (local.get $sum) (i32.load8_u (i32.add (i32.const 100) (local.get $i)))))
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (br $loop3)
    ))

    (i32.store8 (i32.const 0) (i32.const 107))
    (i32.store8 (i32.const 1) (i32.const 52))
    (local.set $len (call $kv_get (i32.const 0) (i32.const 2) (i32.const 100) (i32.const 10)))
    (local.set $i (i32.const 0))
    (block $break4 (loop $loop4
      (br_if $break4 (i32.ge_s (local.get $i) (local.get $len)))
      (local.set $sum (i32.add (local.get $sum) (i32.load8_u (i32.add (i32.const 100) (local.get $i)))))
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (br $loop4)
    ))

    (local.get $sum)
  )

  (func (export "return_eval") (result i32)
    ;; Now it uses hardcoded function. See rustpython_wasm/src/lib.rs file.
    i32.const 50000
    i32.const 1
    call $eval
  )
)

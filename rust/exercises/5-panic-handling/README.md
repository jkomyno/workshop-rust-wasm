# 5 - Panic Handling

- The Rust code for this chapter is in [`rust/5-panic-handling`](./).
- The TypeScript tests for this chapter are in [`nodejs/__tests__/5-panic-handling.test.ts`](../nodejs/__test__/5-panic-handling.test.ts).

> **⚠️ Reminder**
> Run `./watch 5` in both `rust/` and `nodejs/` to automatically recompile the Rust code in this chapter and the corresponding TypeScript tests.

## 5.1 Panics in Rust

As Rust developers, we know that:

- Panics model <strong>unrecoverable errors</strong>
- Panics can be triggered by e.g. calling `.unwrap()` on `None` / `Err` values, accessing out-of-bounds elements, or directly via macros (`panic!`, `todo!`, `unreachable!`)
- They terminate the process abruptly with a stacktrace, unless they are manually caught with [`std::panic::catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)

Here's an example Rust panic stacktrace:

```text
[src/mysql.rs:293:21] called `Result::unwrap()` on an `Err` value:
"Getting create_options from Resultrow as String failed"
   0: migration_engine::set_panic_hook::{{closure}}
  ...
  24: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/.../library/alloc/src/boxed.rs:2000:9
      <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at /rustc/.../library/alloc/src/boxed.rs:2000:9
      std::sys::unix::thread::Thread::new::thread_start
             at /rustc/.../library/std/src/sys/unix/thread.rs:108:17
```

We can clearly see the source file and line where the panic was triggered, the message of the unrecoverable error that caused the panic, and the stacktrace of the functions that were called before the panic.

## 5.2 `Panics` in Rust/Wasm → JavaScript

- Panics cannot be unwinded at all (...yet, see [RFC 2945](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html))
- `wasm32-unknown-unknown` is `panic="abort"` by default
- Once a WebAssembly module panics, it's no longer safe to use (no further function calls can be made to it).
  When such panics happen, you should let your application crash and restart, just like you would do in Erlang or Elixir.
- The stacktraces produced by Wasm are not really useful...

Here's am example Rust/Wasm panic stacktrace as captured by Node.js:

```text
RuntimeError: unreachable
    at rust_panic (wasm://wasm/00a0f2c2:wasm-function[2917]:0x19d798)
    at std::panicking::rust_panic_with_hook::hd000e9fb43b5781d
       (wasm://wasm/00a0f2c2:wasm-function[2056]:0x1934bb)
    ...
    at get_dmmf (wasm://wasm/00a0f2c2:wasm-function[165]:0xdb80d)
    at module2.exports.get_dmmf (.../build/index.js:12094:14)
```

We can make a few observations that hold true no patter the source of the panic:

- The error message is just `"unreachable"` 
- The error bubbled up to the JavaScript runtime is named `RuntimeError`
- The stacktrace includes locations of the function symbols in the Wasm module, but not the Rust source file and line where the panic was triggered

Most "serious" applications will want to submit a bug report to a server when a panic happens, so that the developers can fix the issue and release a new version of the app. The default panic machinery in Rust/Wasm doesn't fit this purpose. Moreover, [`wasm_bindgen`'s docs](https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html?highlight=console_error_panic_hook#cargotoml) currently only recommend to log the panic message to JavaScript's `console.error` via the [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) crate, which is not very useful either.

Luckily, there are ways around that!

## 5.3 `std::panic::set_hook`

While `console_error_panic_hook` doesn't help much per se, we can learn a few things by taking a look at how it works under the hood.
As it turns out, it uses `std::panic::set_hook` to capture the panic message and perform its message printing logic before the panic handler aborts the execution.

This means it's possible to:
- Use `std::panic::set_hook` safely when compiling to Wasm
- Interact with objects from the JavaScript runtime (e.g. `console`) from Rust/Wasm

We basic template for implementing our own Wasm panic handler is:

```rust
use wasm_bindgen::{prelude::wasm_bindgen};

#[wasm_bindgen]
pub fn register_panic_handler() {
    // The panic hook is invoked when a thread panics, but before the panic runtime is invoked.
    std::panic::set_hook(Box::new(|info| {
        todo!();
    }));
}
```

How do we replace that `todo!`? Reminding ourselves of `extern "C"` blocks, we could invoke a globally available function in the JavaScript namespace (that's what I had originally proposed at [EuroRust 2023](https://jkomyno-eurorust-2023.vercel.app/30?clicks=1)).

## 5.4 `JsFunction`: neither `Send` nor `Sync`

Production-grade apps usually have a more sophisticated error reporting system, so we may want to pass a JavaScript function to our Rust/Wasm module, and call that instead.
We can use `js-sys`'s `Function` (which is an `external "C"` binding for the global JavaScript `Function` object) like so:

```diff
use js_sys::{Function as JsFunction};
- use wasm_bindgen::prelude::wasm_bindgen;
+ use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
+
+ struct WasmPanicHandler(JsFunction);
+
+ impl WasmPanicHandler {
+     fn on_panic(&self, info: &std::panic::PanicInfo) {
+         let panic_info: JsString = info.to_string().into();
+         // `JsFunction::call1` yields a `Result`, but we ignore it here
+         // since we're panicking anyway.
+         let this = JsValue::null();
+         let _ = self.0.call1(&this, &panic_info);
+     }
+ }

#[wasm_bindgen]
- pub fn register_panic_handler() {
+ pub fn register_panic_handler(on_panic: JsFunction) {
    // The panic hook is invoked when a thread panics, but before the panic runtime is invoked.
    std::panic::set_hook(Box::new(|info| {
+         handler.on_panic(info);
    }));
}
```

You can find this code in [`rust/5-panic-handling/src/panic_simple.rs`](./src/panic_simple.rs).
The code above would be great, if `JsFunction` was `Send` and `Sync`... but it's not, as Rust clearly tells us:

```rust
error[E0277]: `*mut u8` cannot be shared between threads safely
    --> exercises/5-panic-handling/src/panic_simple.rs:31:26
     |
31   |       std::panic::set_hook(Box::new(|info| {
     |                            ^        ------ within this `[closure@exercises/5-panic-handling/src/panic_simple.rs:31:35: 31:41]`
     |  __________________________|
     | |
32   | |         // handler.on_panic(info);
33   | |         on_panic;
34   | |     }));
     | |______^ `*mut u8` cannot be shared between threads safely
     |
     = help: within `[closure@exercises/5-panic-handling/src/panic_simple.rs:31:35: 31:41]`, the trait `Sync` is not implemented for `*mut u8`
note: required because it appears within the type `PhantomData<*mut u8>`
```

> **ℹ️ Info**
> In general, `wasm-bindgen`'s `JsValue` and any `js-sys` wrapper type around it and `extern "C"` bindings are not thread-safe, so they are neither `Send` nor `Sync`.
> `JsValue` is in fact defined as:
> ```rust
> pub struct JsValue {
>     idx: u32,
>     _marker: marker::PhantomData<*mut u8>, // not at all threadsafe
> }
> ```

With a little bit of effort, we can store the input `JsFunction` in a static `once_cell::sync::OnceCell` variable and use it in our panic handler:

```diff
+ use once_cell::sync::OnceCell;
use js_sys::{Function as JsFunction};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

+ thread_local! {
+     static WASM_PANIC_HANDLER: OnceCell<WasmPanicHandler> = OnceCell::new();
+ }

struct WasmPanicHandler(JsFunction);

impl WasmPanicHandler {
    fn on_panic(&self, info: &std::panic::PanicInfo) {
        let panic_info: JsString = info.to_string().into();
        // `JsFunction::call1` yields a `Result`, but we ignore it here
        // since we're panicking anyway.
        let this = JsValue::null();
        let _ = self.0.call1(&this, &panic_info);
    }
}

#[wasm_bindgen]
pub fn register_panic_handler(on_panic: JsFunction) {
+     WASM_PANIC_HANDLER.with(|handler| {
+         let _ = handler.set(WasmPanicHandler(on_panic));
+     });
+
    // The panic hook is invoked when a thread panics, but before the panic runtime is invoked.
    std::panic::set_hook(Box::new(|info| {
-         handler.on_panic(info);
+         WASM_PANIC_HANDLER.with(|handler| {
+             if let Some(handler) = handler.get() {
+                 handler.on_panic(info);
+             }
+         });
    }));
}
```

You can find this code in [`rust/5-panic-handling/src/panic_untyped.rs`](./src/panic_untyped.rs).

> **⚠️ Note**
> We can't use `once_cell::unsync::OnceCell` as shared static variables must have a type that implements `Sync`,
> which is not the case for `JsFunction`.

## 5.5 Using external functions

`JsFunction` can be replaced by an `extern "C"` public type.
In case of functions being passed as arguments to Rust/Wasm, this gives us room to implement some kind of type safety for our TypeScript colleagues.

`JsFunction` by default is typed as `Function`, which literally means "any function, of any variadic length, of any type". This is not ideal, as TypeScript users are used to types as a documentation (so they know what kind of function and with how many arguments they can share with WebAssembly).

You can however work your way around it via the `typescript_custom_section` and `typescript_type` attributes:

```rust
#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
export type PanicHandler = (panic_info: string) => void;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "PanicHandler")]
    pub type OnPanicJsFunction;
}
```

## 5.6 Exercise

> **🏹 Exercise**
> Implement `panic_typed.rs`, a strongly-typed version of the `panic_untyped.rs` module.
> Name the exported function `register_panic_handler_typed`.
> Ensure the [TypeScript tests]((../nodejs/__test__/5-panic-handling.test.ts)) pass.
>
> Hint: take a look at how `JsFunction::call1` is implemented.

## 5.7 Summary

We have learnt how panics in `wasm32-unknown-unknown` targets are different than in native Rust, how to implement a custom panic handler, and how to decorate external type definitions with TypeScript types.

---

| [⬅️ 4 - `tsify` Datatypes](../4-tsify-types/README.md) | [🏠](/README.md)| [6 - Error handling ➡️](../6-error-handling/README.md)|
|:--------------|:------:|------------------------------------------------:|

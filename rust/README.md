(KEEP)

# 1 - Wasm Preliminaries & Playground Setup

## 1.1 WebAssembly: intro

What is WebAssembly? 

It's a low-level assembly-like language with a compact memory format designed to run alongside JavaScript, the language of the web.

Think of WebAssembly (often abbreviated in Wasm) as a low-level abstraction for the CPU your code is running on, executed on a portable stack-based virtual machine.
It's comparatively fast, as it was designed to offer near-native speed for the web, and it also aims at providing fast startup times with a small memory footprint.
Its linear memory is modeled by a continuos buffer of `u8` bytes that both JavaScript and WebAssembly itself can read and modify.

WebAssembly defines a virtual instruction set architecture which is independent of the OS or CPU we run Wasm on.
However, it lacks support for system call instructions, which limits its ability to interact with I/O in general.

From the point of view of us Rust developers, WebAssembly is just a new compilation target (`wasm32-unknown-unknown`) which requires special care (somewhat similarly to when we write Rust for embedded devices).
We can thus build our WebAssembly-compatible Rust library once, and run it on any JavaScript runtime which includes a WebAssembly virtual machine (Node.js, Deno, your browser, etc).

Rust is a particularly good host language for WebAssembly, because:
- it provides fine-grained levels control with high-level ergonomics
- it's free from non-deterministic garbage collection pauses
- it enables small `.wasm` size (as Rust lacks a runtime)
- enables porting existing crates to JavaScript, so you don't need to reinvent the wheel on JavaScript
- its tooling plays well with the modern JavaScript ecosystem, compared to e.g. C++ and Go

Despite these amenities, both WebAssembly and the Rust tooling are evolving and getting better year after year.

## 1.2 Compile to WebAssembly

Idea: you can write a Rust library that provides functions and constants, mark them as public, and use them programmatically by JavaScript.
It's important to add

```toml
[lib]
# `cdylib` enables `wasm32-*` compilation.
crate-type = ["cdylib"]
```

to the `Cargo.toml` file of your library to prevent compile errors.

> **ℹ️ Info**
> The `cdylib` crate-type option tells the Rust compiler to emit a library dynamically loadable at runtime from another language.
> `cdylib` exposes C API symbols 
> The default crate-type, `rlib`, is only useful when linking with other Rust crates.
> You can read more [here](https://users.rust-lang.org/t/why-do-i-need-to-set-the-crate-type-to-cdylib-to-build-a-wasm-binary/93247).


Compiling a Rust project to WebAssembly via

```sh
cargo build --target wasm32-unknown-unknown
```

generates a `.wasm` file containing WebAssembly bytecode.
However, this output is not directly usable by JavaScript developers and runtime. What then?

## 1.3 `wasm-bindgen`: intro

JavaScript runtimes like Node.js need some glue code to make sense of the WebAssembly's symbols, memory conventions, and type-safe data mangling.
But your JS-oriented colleagues don't want to write weird things like

```js
const mem = getUint8Memory0();
let ptr = malloc(len, 1) >>> 0;
let offset = 0;

for (; offset < len; offset++) {
  const code = arg.charCodeAt(offset);
  if (code > 0x7F) break;
  mem[ptr + offset] = code;
}
```

by hand in order to pass a JavaScript string to WebAssembly's virtual memory!

That's where `wasm-bindgen` comes to the rescue. Taking the WebAssembly bytecode emitted by Rust as an input,
`wasm-bindgen` generates JavaScript and TypeScript bindings for it, facilitating high-level interactions between Wasm modules and JavaScript runtimes.
The `wasm-bindgen` crate and CLI tool also translate Rust and JavaScript datatypes in such a way that WebAssembly can understand (mostly via a reduction to integers, floating points, and pointers).

## 1.4 `wasm-bindgen`: usage example

💻 Open a new terminal tab in the [`rust`](./) folder.

Let's create a playground library from scratch, which you can use to explore the `wasm-bindgen` tool.

```sh
cargo init --lib playground --name w1-playground
```

This creates a new Rust library in the [`rust/playground`](./playground/) folder.

Open its [`Cargo.toml`](./playground/Cargo.toml) file and add:

```diff
[package]
name = "w1-playground"
version = "0.1.0"
edition = "2021"

- # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
+ [lib]
+ # `cdylib` enables `wasm32-*` compilation.
+ crate-type = ["cdylib"]

[dependencies]
+ # use the `wasm-bindgen` dependency declare in the Cargo Workspace (../Cargo.toml)
+ wasm-bindgen.workspace = true
```


Also, add the `playground` crate to the Cargo workspace members in [`rust/Cargo.toml`](./Cargo.toml):

```diff
[workspace]
members = [
  "exercises/*",
  "utils/*",
-   # "playground",
+   "playground",
]
resolver = "2"
```

Add a new `hello_world.rs` file in the [`rust/playground/src`](./playground/src) folder, and write your very first WebAssembly function:

```rust
//! To export a Rust definition to WebAssembly, make sure:
//! - to mark it as `pub`
//! - to annotate it with the `wasm_bindgen` macro

// Import the `wasm-bindgen` macro.
use wasm_bindgen::prelude::wasm_bindgen;

/// When invoked, should print "Hello, world!" to stdout.
#[wasm_bindgen]
pub fn hello_world() {
    println!("Hello, world!");
}
```

Clear the sibling [`lib.rs`](./playground/src/lib.rs) file, and don't forget to import this module you've just created:

```diff
// rust/playground/src/lib.rs
+ pub mod hello_world;
```

Assuming your terminal tab is still open in the `rust` folder, compile the playground we've just created to WebAssembly with:

```sh
cargo build \
  -p w1-playground \
  --target wasm32-unknown-unknown
```

> **⚠️ Reminder**
> `cargo build`, by default, builds crates in "debug" mode. This is fine for development and learning purposes.
> In practice, however, don't forget to build your code with the `--release` flag, as debug builds for WebAssembly provide a dreadful performance and an extra-large bundle size.
> For example, after repeating the command above with `--release`, we get:
>
> ```sh
> ❯ du -h ./target/wasm32-unknown-unknown/debug/w1_playground.wasm
> 2.2M    ./target/wasm32-unknown-unknown/debug/w1_playground.wasm
>
> ❯ du -h ./target/wasm32-unknown-unknown/release/w1_playground.wasm
> 136K    ./target/wasm32-unknown-unknown/release/w1_playground.wasm
> ```
>
> The `release` version is 16x smaller than the `debug` one!

Now that we have generated the `.wasm` bytecode, let's create the JavaScript bindings for it with the `wasm-bindgen` CLI tool.
From `wasm-bindgen --help`, we can figure out that a typical invocation of `wasm-bindgen` looks like:

```sh
wasm-bindgen \
  --target <JS target ("bundler", "nodejs", "web", ...)> \
  --out-dir <output directory> \
  <input .wasm file>
```

We run `wasm-bindgen` as:

```sh
wasm-bindgen \
  --target bundler \
  --out-dir ./playground/wasm \
  ./target/wasm32-unknown-unknown/debug/w1_playground.wasm
```

> **ℹ️ Info**
> The `--target` flag of `wasm-bindgen` specifies how the generated JavaScript bindings should be structured.
> The JavaScript ecosystem is very diverse due to competing non-standardized module systems,
> but for modern web development where third-party JavaScript code bundlers are the norm,
> the `bundler` target is the most recommended choice.
> You can read more [here](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html).

Our `playground` folder should now look like:

```sh
❯ tree playground

playground
├── Cargo.toml
├── src
│   ├── hello_world.rs
│   └── lib.rs
└── wasm
    ├── w1_playground.d.ts
    ├── w1_playground.js
    ├── w1_playground_bg.js
    ├── w1_playground_bg.wasm
    └── w1_playground_bg.wasm.d.ts
```

> **ℹ️ Info**
> The `*_bg.*` files are the glue code generated by `wasm-bindgen` to make the WebAssembly module usable by JavaScript.
> Note that the `*_bg.wasm` file is NOT the same as the `*.wasm` file we obtained with `cargo build` earlier.
>
> You can verify this by yourself by running
> ```sh
> # `.wat` version of the `*.wasm` file generated by `cargo build`
> wasm2wat ./target/wasm32-unknown-unknown/release/w1_playground.wasm > ./playground/wasm/w1_playground.release.wat
>
> # `.wat` version of the `*.wasm` file generated by `wasm-bindgen`
> wasm2wat ./playground/wasm/w1_playground_bg.wasm > ./playground/wasm/w1_playground.bg.wat
> ```
> (which converts the `.wasm` bytecode to a human-readable `.wat` file) and comparing the two `.wat` files.

This means we can import `./wasm/w1_playground.js` in JavaScript: it's finally time to run our WebAssembly module in Node.js!

## 1.5 Run WebAssembly in Node.js

💻 Open a new terminal tab in the [`nodejs`](../) folder.

You'll notice a [`./src/playground.ts`](../nodejs/src/playground.ts) script there:

```typescript
// nodejs/src/playground.ts
import * as wasm from '../../rust/playground/wasm/w1_playground'

wasm.hello_world()
```

Let's run it with:

```sh
❯ pnpm --silent dev ./src/playground.ts
```

What? There's no "hello world" output! In fact, there's no output at all.
Is this TypeScript file even running? Let's add a couple of `console.log`s for good measure:

```diff
// nodejs/src/playground.ts
import * as wasm from '../../rust/playground/wasm/w1_playground'

+ console.log('before wasm')
wasm.hello_world()
+ console.log('after wasm')
```

and try again:

```sh
❯ pnpm --silent dev ./src/playground.ts
before wasm
after wasm
```

![Mumble mumble](https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExNjU3eDBiYXZjcnZ5ZHFlM2x1b3l0ZmY5a2syODNwemh5N3FsbTB5MCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/CaiVJuZGvR8HK/giphy.gif)

Wait! Earlier we said that WebAssembly has no direct access to the I/O of the host environment.
In fact, `println!` (which expands to `std::io::_print()`) doesn't work on `wasm32-unknown-unknown` targets.

We could just settle for a Rust function that returns a string, and print it from JavaScript via `console.log`:

```diff
// rust/playground/src/hello_world.rs

- /// When invoked, should print "Hello, world!" to stdout.
#[wasm_bindgen]
- pub fn hello_world() {
+ pub fn hello_world() -> String {
+     println!("Hello, world!");
+     "Hello, world!".to_string()
}
```

```diff
// nodejs/src/playground.ts

import * as wasm from '../../rust/playground/wasm/w1_playground'

- console.log('before wasm')
- wasm.hello_world()
+ console.log(
+   wasm.hello_world(),
+ )
- console.log('after wasm')
```

which, sure enough, works as intended (after a round of recompilation):

```
# In the 'nodejs' folder
❯ pnpm --silent dev ./src/playground.ts
Hello, world!
```

What if we wanted to print something from Rust, using the same `console.log` function, but without having to change the JavaScript code?

We can do so by leveraging the fact that WebAssembly has access to the JavaScript runtime's global namespace - which includes the `console` object -. In fact, the `crate-type = "cdylib"` allows `extern "C"` blocks that reference datatypes and functions that are not available at compile time, but that will be at runtime. This is essentially how crates like [`js-sys`](https://docs.rs/js-sys/latest/js_sys/) (which expose Rust bindings to the JavaScript built-in objects) are built under the hood.

## 1.6 `extern "C"` functions in WebAssembly

`extern "C"` enables Rust to use a cross-platform C ABI to refer to definitions at runtime (see more [here](https://doc.rust-lang.org/reference/items/external-blocks.html#abi)).
When combined with `wasm-bindgen`, there are two main attributes to keep in mind:

- `js_namespace`: it refers to the global JavaScript namespace that we're binding to
- `js_name` refers to the name of the function we're binding to. If not explicitly set, it is the name used by the Rust function.

Add a new `hello_console.rs` file in the [`rust/playground/src`](./playground/src) folder (and import it in the sibling `lib.rs` file):

```rust
//! To export a Rust definition to WebAssembly, make sure:
//! - to mark it as `pub`
//! - to annotate it with the `wasm_bindgen` macro

// Import the `wasm-bindgen` macro.
use wasm_bindgen::prelude::wasm_bindgen;

mod js {
    use super::*;

    // `extern "C"` uses the C ABI, allowing us to refer to global JS definitions
    // directly from Rust.
    // See more at https://doc.rust-lang.org/reference/items/external-blocks.html#abi.
    #[wasm_bindgen]
    extern "C" {
        // Use `js_namespace` here to bind `console.log(..)` instead of just
        // `log(..)`
        #[wasm_bindgen(js_namespace = console, js_name = "log")]
        pub fn console_log(s: &str);
    }
}

/// When invoked, should print "Hello, console!" to stdout.
#[wasm_bindgen]
pub fn hello_console() {
    js::console_log("Hello, console!");
}
```

```diff
// rust/playground/src/lib.rs
+ pub mod hello_console;
```

We can try this out by changing the `./src/playground.ts` file to

```diff
// nodejs/src/playground.ts

import * as wasm from

+ wasm.hello_console()
- console.log(
-   wasm.hello_world(),
- )
```

After recompiling, we get:

```
# In the 'nodejs' folder
❯ pnpm --silent dev ./src/playground.ts
Hello, console!
```

> **ℹ️ Info**
> In practice, you may want to use crates like [`wasm-rs-dbg`](https://github.com/wasm-rs/dbg) to print to the JavaScript console from Rust, but this is a topic for another time.

## 1.7 Summary

At this point, you should have a basic understanding of what `wasm-bindgen` is and how to apply it to functions, both locally-defined or `extern`-al.

Get ready for the [next section](./exercises/2-wasm-bindgen-types/README.md), where will explore how to integrate `wasm-bindgen` with common datatypes that you may encounter in your daily WebAssembly developer life.

---

| [🏠](/README.md)| [2 - `wasm-bindgen` datatypes ➡️](./exercises/2-wasm-bindgen-types/README.md)|
|:------:|------------------------------------------------:|

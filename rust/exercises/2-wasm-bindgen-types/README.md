# 2 - `wasm-bindgen` datatypes

- The Rust code for this chapter is in [`rust/2-wasm-bindgen-types`](./).
- The TypeScript tests for this chapter are in [`nodejs/__tests__/2-wasm-bindgen-types.test.ts`](../nodejs/__test__/2-wasm-bindgen-types.test.ts).

> **⚠️ Reminder**
> Run `./watch 2` in both `rust/` and `nodejs/` to automatically recompile the Rust code in this chapter and the corresponding TypeScript tests.

## 2.1 `wasm-bindgen`: Scalar types

In the previous lesson, we've learned how to export Rust functions to WebAssembly via the `#[wasm_bindgen]` macro.
But do we have any kind of restriction on the types of the arguments and return values of these functions?
Let's start with the simplest case: scalar types.

Scalar types like `u8`, `i32`, `u128`, `f64`, `bool`, `char`, are all supported by `wasm-bindgen` out of the box, both as input and output values. `wasm-bindgen` also automatically generates TypeScript bindings for them.

For example,

```rust
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn inc_biguint(x: u64) -> u64 {
    x + 1
}
```

results in the following TypeScript definition:

```typescript
function inc_biguint(a: number): number;
```

## 2.2 `wasm-bindgen`: String types

Strings are a bit more complicated:
- `String` is supported out of the box as both an input and output type
- `&str` is a valid input type, but not a valid output type

Moreover, Rust and JavaScript encode strings differently:
- Rust uses UTF-8
- JavaScript uses UTF-16

This implies that e.g. measuring the length of a string in Rust or in JavaScript may yield different results!

## 2.3 `wasm-bindgen`: Enum types

`wasm-bindgen` supports `enum`s, but only if they are C-like (i.e., they have no variants, see https://github.com/rustwasm/wasm-bindgen/issues/2407).
Enums must explicitly be marked via `#[wasm_bindgen]`. Third-party C-like enums not marked with `#[wasm_bindgen]` cannot be used as input or output types: they must be converted into a datatype that `wasm-bindgen` has visibility on.

A couple of enum variants from the Rust standard library are however supported out of the box by `wasm-bindgen`, due to their popularity: `Option<T>` and `Result<T, E>`.

## 2.4 `wasm-bindgen`: Struct types

`wasm-bindgen` supports `struct`s, but they must be explicitly marked via `#[wasm_bindgen]`.
They must be publicly visible, and their fields of interest to JavaScript users should be public as well.
In JavaScript, they are implemented as a class with a `Copy`-derived getter for each public field.

Should struct contain public fields that do not implement the `Copy` trait (like `String`), you need to apply `#[wasm_bindgen(getter_with_clone)]` on the field itself (see [more](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/getter_with_clone.html)) (which uses the `Clone` trait on the getter for that particular field instead).

## 2.5 `wasm-bindgen`: Vector types

Rust vectors of numbers are translated into `TypedArray` buffers in JavaScript, e.g.:
- `Vec<u8>` becomes `Uint8Array`
- `Vec<i32>` becomes `Int32Array`
- `Vec<f64>` becomes `Float64Array`

Vectors of non-primitive types (`String` included) are not supported.
Moreover, nested vectors (`Vec<Vec<T>>` for some type `T`) and tuples are not supported.

In general, `wasm-bindgen` does not support `Box`-ed types at all.

## 2.6 `wasm-bindgen`: lifetimes

The `wasm-bindgen` macro does not support lifetimes in the type signature of an exported function, as they are a Rust-specific concept that does not apply to e.g. JavaScript (see more [here](https://github.com/rustwasm/wasm-bindgen/issues/1187)).

Also, generic types are not supported. You can however use a concrete instantiation of a generic type, or use `wasm-bindgen`'s `JsValue` type (which is a catch-all type for any JavaScript value, and comes with several `Into<..>` trait implementations).

## 2.7 Exercise

> **🏹 Exercise**
> Port the `enum_c.rs`, `scalars.rs`, `lib.rs`, modules to WebAssembly, ensuring the [TypeScript tests]((../nodejs/__test__/2-wasm-bindgen-types.test.ts)) pass.

## 2.8 Summary

So far, we've learnt how to use `wasm-bindgen` with several different datatypes, and understood a few of its limitations.

Get ready for the [next section](../3-wasm-bindgen-classes/README.md), where will explore how to integrate `wasm-bindgen` with classes!

---

| [⬅️ 1 - Wasm Preliminaries & Playground Setup](../../README.md) | [🏠](/README.md)| [3 - `wasm-bindgen` Classes ➡️](../3-wasm-bindgen-classes/README.md)|
|:--------------|:------:|------------------------------------------------:|

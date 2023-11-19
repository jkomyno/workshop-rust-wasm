# 4 - `tsify` datatypes

- The Rust code for this chapter is in [`rust/4-tsify-types`](./).
- The TypeScript tests for this chapter are in [`nodejs/__tests__/4-tsify-types.test.ts`](../nodejs/__test__/4-tsify-types-classes.test.ts).

> **⚠️ Reminder**
> Run `./watch 4` in both `rust/` and `nodejs/` to automatically recompile the Rust code in this chapter and the corresponding TypeScript tests.

## 4.1 `serde` + `tsify` = ❤️

In a previous lesson, we've seen how `wasm-bindgen` is somewhat limited in understanding more complex Rust datatypes, such as `enum` variants, generic vectors, and others.

A few crates can be used to overcome this limitation (bypassing `wasm-bindgen`' FFI limitations and de-serialization rules), like [`serde_json`](https://github.com/serde-rs/json) and [`serde-wasm-bindgen`](https://github.com/cloudflare/serde-wasm-bindgen), but they all fall short when it comes to TypeScript support, which nowadays is a de facto standard in the JavaScript ecosystem.
What thing do these crates have in common? They are built on top of `serde`, the popular serialization/deserialization framework for Rust.

There's one particular library that emerged in the past couple of years, which combined the power of `serde` with the TypeScript ecosystem: [`tsify`](https://github.com/madonoharu/tsify), which I've talked about at [EuroRust 2022](https://jkomyno-eurorust-2022.vercel.app/25).

It provides two main features:
- a `Tsify` derive macro (to be applied alongside `Serialize`/`Deserialize` derives from `serde`)
- a `tsify` macro to generate Rust code for serialization/deserialization.

In particular, you should apply the `#[tsify(into_wasm_abi)]` attribute when deriving `Serialize`, and the `#[tsify(from_wasm_abi)]` attribute when deriving `Deserialize`.
Most importantly, you should not apply `#[wasm_bindgen]` to the same struct, as that would conflict with `tsify`'s code generation (and result in a compile error). `tsify` still use `wasm-bindgen` under the hood.

## 4.2 `tsify` in action

Open the exercise files and read the comments to see how `tsify` can be used to overcome `wasm-bindgen`'s limitations.

## 4.3 Summary

In this chapter, we've briefly mentioned some alternatives to `wasm-bindgen` for dealing with more complex Rust datatypes as input and output arguments.
In particular, we've seen how `tsify` can be used to generate TypeScript code for serialization/deserialization, and how it can be used to overcome `wasm-bindgen`'s limitations.

---

| [⬅️ 3 - `wasm-bindgen` Classes](../3-wasm-bindgen-classes/README.md) | [🏠](/README.md)| [5 - Panic handling ➡️](../5-panic-handling/README.md)|
|:--------------|:------:|------------------------------------------------:|

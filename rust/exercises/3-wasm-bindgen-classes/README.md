# 3 - Wasm Classes

- The Rust code for this chapter is in [`rust/3-wasm-bindgen-classes`](./).
- The TypeScript tests for this chapter are in [`nodejs/__tests__/3-wasm-bindgen-classes.test.ts`](../nodejs/__test__/3-wasm-bindgen-classes.test.ts).

> **⚠️ Reminder**
> Run `./watch 3` in both `rust/` and `nodejs/` to automatically recompile the Rust code in this chapter and the corresponding TypeScript tests.

## 3.1 From Rust struct methods to JavaScript classes

In the previous lesson, we've seen how to define Rust functions that
can be used by WebAssembly with a few different datatypes. However, Rust logic - especial in apps dealing with domain modeling - is often times expressed via `struct`s and related methods, rather than free functions.

`wasm_bindgen` allows Rust structs to be used as classes in JavaScript: let's learn how!

💻 Open a new terminal tab in the [`rust`](./) folder.

For example, consider a scenario in which a JavaScript wants to use a Rust date-utility library with this functionality:

```typescript
const date = new wasm.Date(2023, 11, 18) // year, month, day
console.log(date.fmt_italian()) // prints "18/11/2023", in "DD/mm/YYYY" format
// console.log(date.next_year()) // increments the year 
```

In pure Rust, we could write this as:

```rust
pub struct Date {
  year: u16,
  month: u8,
  day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    // Returns the date in "DD/mm/YYYY" format
    pub fn fmt_italian(&self) -> String {
        format!("{:02}/{:02}/{:04}", &self.day, &self.month, &self.year)
    }
}
```

You can find this code in [`rust/3-wasm-bindgen-classes/src/date.rs`](./src/date.rs).

To enable WebAssembly usage of this class, we need

```diff
+ use wasm_bindgen::prelude::wasm_bindgen;

+ #[wasm_bindgen]
pub struct Date {
  year: u16,
  month: u8,
  day: u8,
}

+ #[wasm_bindgen]
impl Date {
    + #[wasm_bindgen(constructor)]
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    // Returns the date in "DD/mm/YYYY" format
    + #[wasm_bindgen]
    pub fn fmt_italian(&self) -> String {
        format!("{:02}/{:02}/{:04}", &self.day, &self.month, &self.year)
    }
}
```

Let's compile it:

```sh
./build.sh 3
```

No problems so far; we even managed to compile fine with a `&self` reference in `Date::fmt_italian`. Now let's spice things up a bit.

## 3.2 Methods returning references

Consider a struct as follows:

```rust
pub struct Event {
    pub name: String,
    pub year: u16,
}
```

Let's say that the `name` field must always be specified by a user, whereas the `year` defaults to the current year. We could implement the builder pattern for this:

```rust
#[derive(Default)]
pub struct EventBuilder {
    name: String,
    year: Option<u16>,
}

impl EventBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn with_year(&mut self, year: u16) -> &Self {
        self.year = Some(year);
        self
    }

    pub fn build(self) -> Event {
        Event {
            name: self.name,
            year: self.year.unwrap_or(2023),
        }
    }
}
```

You can find this code in [`rust/3-wasm-bindgen-classes/src/date.rs`](./src/date.rs).

The peculiar thing about `EventBuilder` is that it returns `&Self`.
While this is totally fine in Rust, it's forbidden in WebAssembly:

```
error: cannot return a borrowed ref with #[wasm_bindgen]
  --> exercises/3-wasm-bindgen-classes/src/event.rs:35:47
   |
35 |     pub fn with_year(&mut self, year: u16) -> &Self {
   |                                               ^^^^^

```

This is due to `wasm-bindgen` preventing dangling references.
This means we can't use fluent Rust APIs in WebAssembly directly! We can however drop the reference,
and return a copy of `Self`.

## 3.3 Exercise

> **🏹 Exercise**
> Port the `event.rs` module to WebAssembly, ensuring the [TypeScript tests]((../nodejs/__test__/3-wasm-bindgen-classes.test.ts)) pass.

Recall that `wasm-bindgen` generates a getter method for each public field of a struct, which requires such field to implement `Copy`. What about nested fields that are not `Copy`, like `String`?
You can get around the issue by using `#[wasm_bindgen(getter_with_clone)]` on the field itself (see [more](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/getter_with_clone.html)).

Also, to get automatic `toJSON` and `toString` methods that display all public fields of structs marked visible to `wasm-bindgen`, you can use the `#[wasm_bindgen(inspectable)]` attribute (see [more](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/inspectable.html)).

## 3.4 Summary

You now know how to initialise Rust structs from JavaScript, and how to call their methods.
We've presented both "return by reference" and "return by value" scenarios, and how to deal with them.
You've also learnt a few more useful `wasm-bindgen` attributes along the way.

Now, get ready for the [next section](../4-tsify-types/README.md), where you will learn how to use `serde` and `tsify` to use more advanced data structures in WebAssembly.

| [⬅️ 2 - `wasm-bindgen` types](../2-wasm-bindgen-types/README.md) | [🏠](/README.md)| [4 - Tsify types ➡️](../4-tsify-types/README.md)|
|:--------------|:------:|------------------------------------------------:|

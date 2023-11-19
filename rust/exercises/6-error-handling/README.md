# 6 - Error Handling

- The Rust code for this chapter is in [`rust/6-error-handling`](./).
- The TypeScript tests for this chapter are in [`nodejs/__tests__/6-error-handling.test.ts`](../nodejs/__test__/6-error-handling.test.ts).

> **⚠️ Reminder**
> Run `./watch 6` in both `rust/` and `nodejs/` to automatically recompile the Rust code in this chapter and the corresponding TypeScript tests.

## 6.1 Errors in Rust

As Rust developers, we know that:

- Rust models computations that could potentially fail as functions returning `Result<T, E>`
- Under the hood, `Result<T, E>` is either a wrapper for a generic `T` type on successful runs, or a wrapper for a generic error `E` otherwise.
- These errors are recoverable, so the same functions can be retried later on.
- Rust uses pattern matching or the "`?`" operator to deal with failure

Here's an example of a Rust function that yields a `Result`:

```rust
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("division by zero");
    }

    Ok(a / b)
}

pub fn run() -> Result<i32, &'static str> {
    let value = divide(10, 2)?;
    Ok(value)
}
```

## 6.2 Errors in JavaScript

JavaScript (and consequently TypeScript), on the other hand, adopts a radically different approach to error handling, modeled around `try / catch` and `Error`s (runtime exceptions) that, if not caught, causes the process to halt with a failure:

```typescript
function divide(a: number, b: number): number {
  if (b === 0) {
    throw new Error('division by zero')
  }

  return a / b
}

try {
  const result = divide(10, 2)
  console.log(`Success: ${result}`)
} catch (error) {
  const e = error as Error
  console.log(`Failure: ${e.message}`)
}
```

But that's not all. JavaScript developers (and the libraries in the JS ecosystem) frequently require some conventions to be held when it comes to error handling - conventions that may not be obvious to Rust developers integrating WebAssembly modules into JavaScript applications.

## 6.3 Error conventions in JavaScript

- Errors should be instances of the `Error` class, or a subclass of it.
- Errors should describe what caused the runtime disruption of execution in the `message` property, which is always a `string`.
- [Non-standard] Additional information related to the error can be added to the `meta` property.

## 6.4 Convention violations with `wasm-bindgen`

`Result::Err` isn't necessarily translated to an instance of `Error` in JavaScript.

For example, functions that return `Result<_, String>` result in a `string` being thrown in JavaScript, which is not an instance of `Error`. For instance, consider a Rust function that parses a JSON-encoded string into a Rust struct:

```rust
use wasm_bindgen::{prelude::wasm_bindgen};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Event {
    name: String,
    year: u16,
}

#[wasm_bindgen(js_name = "parseWithStringError")]
pub fn parse_with_string_error(event: &str) -> Result<Event, String> {
    let event: Event = serde_json::from_str(event).map_err(|e| e.to_string())?;
    Ok(event)
}
```

In TypeScript tests, this results in:

```typescript
import { expect, describe, test, assert } from 'vitest'
import * as wasm from '../wasm/6-error-handling/6-error-handling'

// Note: "year" is missing!
const eventAsStr = '{ "name": "EuroRust" }'

describe('demo-errors', () => {
  describe('parseWithStringError', () => {
    test('throws a `String`', () => {
      try {
        wasm.parseWithStringError(eventAsStr)
        assert(false, 'this should fail')
      } catch (error) {
        const e = error as string

        assert(typeof e === 'string') // <===
        expect(e).toMatchInlineSnapshot('"missing field `year` at line 1 column 22"')
      }
    })
  })
})
```

The following `Result<_, E>` types are all `wasm-bindgen` compatible and result in an `Error` being thrown:
- `Result<_, wasm_bindgen::JsError>`, which can be initialized via `JsError::from(e)`, where `e` is the error message
- `Result<_, dyn Error + Display>`, which allows for a custom formatted message.

Using any of the above types is thus the recommended way to throw JavaScript errors from Rust functions, but what about the other convention of adding additional, non-standard fields to the `Error` object?

Discover how `extern "C"` and a macro can make this easy in [./src/lib.rs](./src/lib.rs)!

## 6.5 Summary

We have learnt that the error handling ergonomics of JavaScript are radically different than Rust, which is something to be mindful of when developing WebAssembly modules destined to be consumed by JavaScript applications.

---

| [⬅️ 5 - Panic handling](../5-panic-handling/README.md) | [🏠](/README.md)| [Demo ➡️](../7-demo/)|
|:--------------|:------:|------------------------------------------------:|

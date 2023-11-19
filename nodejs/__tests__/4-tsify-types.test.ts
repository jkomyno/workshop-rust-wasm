import { expect, describe, test } from 'vitest'
import * as wasm from '../wasm/4-tsify-types/4-tsify-types'

describe('3-tsify-types', () => {
  describe('scalars', () => {
    test('scalars', () => {
      const scalars: wasm.Scalars = {
        i32: 1,
        i128: 99999999999999999n,
        bool: true,
        str: 'hello 👋🏻😎',
        char: 'l', // try changing this to a string with more than one character
      }

      expect(wasm.get_char_from_scalars(scalars)).toEqual('l')
      expect(wasm.get_str_from_scalars(scalars)).toEqual('hello 👋🏻😎')
    })
  })

  describe('maps and vectors', () => {
    test('index', () => {
      const frequencyMap: Map<string, number> = new Map(Object.entries({
        'Python': 3,
        'Rust': 10,
        'TypeScript': 8,
        'Golang': 6,
        'WebAssembly': 9,
      }))

      expect(wasm.top_frequent(frequencyMap, 3)).toMatchInlineSnapshot(`
        [
          "Rust",
          "WebAssembly",
          "TypeScript",
        ]
      `)

      expect(wasm.top_frequent(frequencyMap, 2)).toMatchInlineSnapshot(`
      [
        "Rust",
        "WebAssembly",
      ]
    `)
    })
  })

  test('enum variants (ADT)', () => {
    const eitherOk = wasm.either_ok(1)
    expect(eitherOk).toMatchInlineSnapshot(`
      {
        "_tag": "Ok",
        "value": 1,
      }
    `)
    expect(wasm.either_to_string(eitherOk)).toBe('Ok(1)')

    const eitherErr = wasm.either_err('<error message>')
    expect(eitherErr).toMatchInlineSnapshot(`
      {
        "_tag": "Err",
        "value": "<error message>",
      }
    `)
    expect(wasm.either_to_string(eitherErr)).toBe('Err(<error message>)')
  })
})

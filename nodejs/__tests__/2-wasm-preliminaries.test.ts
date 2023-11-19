import { expect, describe, test } from 'vitest'
import * as wasm from '../wasm/2-wasm-bindgen-types/2-wasm-bindgen-types'

describe('playground-wasm-bindgen', () => {
  describe('functions', () => {
    test('inc_biguint: u64 in Rust is bigint in TS', () => {
      expect(wasm.inc_biguint(1n)).toEqual(2n)

      // with negative inputs, it wraps around without explicit errors
      expect(wasm.inc_biguint(-2n)).toEqual(18446744073709551615n)
    })

    test('inc_uint: u32 in Rust is number in TS', () => {
      expect(wasm.inc_uint(2)).toEqual(3)

      // with negative inputs, it wraps around without explicit errors
      expect(wasm.inc_uint(-2)).toEqual(4294967295)
    })

    test('inc_int: i32 in Rust is number in TS', () => {
      expect(wasm.inc_int(-1)).toEqual(0)
    })

    test('inc_uint: u8 in Rust is number in TS', () => {
      expect(wasm.inc_byte(128)).toEqual(129)

      // with negative inputs, it wraps around without explicit errors
      expect(wasm.inc_byte(-2)).toEqual(255)
    })

    test('f64 in Rust is number in TS', () => {
      expect(wasm.inc_f64(-1.5)).toEqual(-0.5)
    })

    describe('C-style enum in Rust is enum in TS', () => {
      test('enum_to_string', () => {
        expect(wasm.enum_to_string(wasm.DbProvider.Postgres)).toEqual('postgres')
        expect(wasm.enum_to_string(wasm.DbProvider.MySQL)).toEqual('mysql')
        expect(wasm.enum_to_string(wasm.DbProvider.SQLite)).toEqual('sqlite')
      })

      test('enum_from_string', () => {
        expect(wasm.enum_from_string('postgres')).toEqual(wasm.DbProvider.Postgres)
        expect(wasm.enum_from_string('mysql')).toEqual(wasm.DbProvider.MySQL)
        expect(wasm.enum_from_string('sqlite')).toEqual(wasm.DbProvider.SQLite)
      })
    })

    test('Option<i32> in Rust is number | undefined in TS', () => {
      expect(wasm.inc_int_maybe()).toEqual(undefined)
      expect(wasm.inc_int_maybe(undefined)).toEqual(undefined)
      expect(wasm.inc_int_maybe(1)).toEqual(2)
    })

    test('Result<i32, String> in Rust returns a number or throws a string in TS', async () => {
      expect(wasm.inc_int_or_fail(1)).toEqual(2)

      try {
        wasm.inc_int_or_fail()
        expect(true).toEqual(false) // should not reach here
      } catch (e) {
        expect(typeof(e)).toEqual('string') // not an Error
      }
    })

    describe('scalars', () => {
      test('it needs a class instance', async () => {
        try {
          const scalars: wasm.Scalars = {
            free: () => {},
            n: 1,
            id: 1n,
            letter: 'a',
            toggle: true,
          }
  
          wasm.get_letter(scalars)
        } catch (error) {
          const e: Error = error

          expect(e.message).toMatchInlineSnapshot('"expected instance of Scalars"')
        }
      })

      test('get_letter', () => {
        const scalars = wasm.new_scalars(1, 1n, 'a', true)
        expect(wasm.get_letter(scalars)).toEqual('a')
      })
    })
  })
})

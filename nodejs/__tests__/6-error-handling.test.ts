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

        assert(typeof e === 'string')
        expect(e).toMatchInlineSnapshot('"missing field `year` at line 1 column 22"')
      }
    })
  })

  describe('parseWithError', () => {
    test('throws an `Error`', () => {
      try {
        wasm.parseWithError(eventAsStr)
        assert(false, 'this should fail')
      } catch (error) {
        const e = error as Error

        assert(e instanceof Error)
        expect(e.name).toEqual('Error')
        expect(e.message).toMatchInlineSnapshot('"missing field `year` at line 1 column 22"')
      }
    })
  })

  describe('parseWithCustomError', () => {
    test('throws an `Error` with a custom message', () => {
      try {
        wasm.parseWithCustomError(eventAsStr)
        assert(false, 'this should fail')
      } catch (error) {
        const e = error as Error

        assert(e instanceof Error)
        expect(e.name).toEqual('Error')
        expect(e.message).toMatchInlineSnapshot('"[CustomError] missing field `year` at line 1 column 22"')
      }
    })
  })

  describe('parseWithErrorObjectMacro', () => {
    test('throws an `Error` with custom properties', () => {
      try {
        wasm.parseWithErrorObjectMacro(eventAsStr)
        assert(false, 'this should fail')
      } catch (error) {
        const e = error as Error & { code: string, someMessage: never, someNumber: number }

        console.log('e', Object.entries(e))

        assert(e instanceof Error)
        expect(e.name).toEqual('Error')
        expect(e.message).toMatchInlineSnapshot('"missing field `year` at line 1 column 22"')
        expect(e.code).toEqual('ERROR_OBJECT_CODE')
        expect(e.someMessage).toBe(undefined)
        expect(e.someNumber).toBe(42)
      }
    })
  })
})

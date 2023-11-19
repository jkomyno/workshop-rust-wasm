import { expect, describe, test } from 'vitest'
import * as wasm from '../wasm/3-wasm-bindgen-classes/3-wasm-bindgen-classes'

describe('1-wasm-classes', () => {
  describe('date', () => {
    test('can be constructed and yields date in format "DD/mm/YYYY"', () => {
      const date = new wasm.Date(2023, 11, 19)
      expect(date.fmtItalian()).toEqual('19/11/2023')
    })
  })
  
  describe('event', () => {
    test('event builder', () => {
      const event = new wasm
        .EventBuilder('RustLab')
        .with_year(2023)
        .build()

      expect(event.fmt()).toEqual('RustLab (2023)')
    })

    test('event is introspectable', () => {
      const event = new wasm
        .EventBuilder('RustLab')
        .with_year(2023)
        .build()

      expect(event.toJSON()).toEqual({ year: 2023, name: 'RustLab' })
    })
  })
})

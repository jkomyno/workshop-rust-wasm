import { expect, describe, test } from 'vitest'
import * as wasm from '../wasm/5-panic-handling/5-panic-handling'

describe('4-panic-handling', () => {
  describe('panic', () => {
    test('calling `init_wasm` allows decoding panic info', async () => {

      let panicInfo = undefined
      
      // Set up a panic hook to capture the Rust/Wasm panic info.
      const onPanic = (panicInfoFromWasm: string) => {
        panicInfo = panicInfoFromWasm
      }
  
      // Tell the Wasm module to call `onPanic` when a panic happens.
      // What happens if change the signature of `onPanic`?
      // What if you call `wasm.register_panic_handler_typed` instead?
      wasm.register_panic_handler_untyped(onPanic)
  
      try {
        // Trigger `panic!('<your panic message>')` in Rust/Wasm.
        wasm.trigger_panic('<your panic message>')

        // Test that JS caught the panic and entered the `catch` block.
        expect.unreachable()
      } catch (error) {
        const e = error as Error
        expect(e.name).toEqual('RuntimeError')

        // Panic errors captured by the JS runtime are not informative.
        // They only say `unreachable`!
        expect(e.message).toEqual('unreachable')

        expect(panicInfo).toMatchInlineSnapshot(`
          "panicked at exercises/5-panic-handling/src/lib.rs:$LINE:$COL:
          \\"<your panic message>\\""
        `)

        // Uncomment to see the stack trace:
        // expect(e.stack).toMatchInlineSnapshot()
      }
    })
  })

  /**
   * This is just a JS utility to hide the line and column of panic messages rom the snapshot tests,
   * making them predictable.
   */
  expect.addSnapshotSerializer({
    serialize(val, config, indentation, depth, refs, printer) {
      return printer(val.replace(/(.*\.rs)(:\d*:\d*)/g, '$1:$LINE:$COL'), config, indentation, depth, refs)
    },
    test(val) {
      return typeof val === 'string' && (val.match(/(.*\.rs)(:\d*:\d*)/)?.length ?? 0) > 0
    },
  })
})

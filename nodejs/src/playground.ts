// You can run this script in two ways (from the `nodejs` directory):
// - via on-the-fly transpilation:
//   - `pnpm dev ./src/playground.ts`
// - via pre-transpilation:
//   - `pnpm build`
//   - `node ./build/playground.js`
import * as wasm from '../../rust/playground/wasm/w1_playground'

wasm.hello_world()

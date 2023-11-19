// You can run this script in two ways (from the `nodejs` directory):
// - via on-the-fly transpilation:
//   - `pnpm dev ./src/playground.ts`
// - via pre-transpilation:
//   - `pnpm build`
//   - `node ./build/playground.js`

import * as wasm from '../wasm/7-demo-psl-wasm/7-demo-psl-wasm'

const __dirname = new URL('.', import.meta.url).pathname

// const schema = fs.readFileSync(path.join(__dirname, 'schema.prisma'), 'utf-8')

try {
  wasm.validate(/* prisma */`
  
    datasource db {
      provider = "mysql"
      url      = env("DATABASE_URL")
    }
  
  `)
} catch (error) {
  const e: Error = error

  console.error(e.message)
  console.log(e)
}

import * as esbuild from 'esbuild'
import { wasmLoader } from 'esbuild-plugin-wasm'

esbuild.build({
  plugins: [
    // `esbuild`'s counterpart to `--experimental-wasm-modules`
    wasmLoader()
  ],
  
  outdir: './build',
  target: 'esnext',
  entryPoints: ['src/playground.ts'],
  bundle: true,
  format: 'esm',
  platform: 'node',
})

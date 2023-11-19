/**
 * Configuration file for `vitest`, a test runner for Node.js.
 */

/// <reference types="vitest" />
import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'

export default defineConfig({
  plugins: [
    // `vite`'s counterpart to `--experimental-wasm-modules`
    wasm(),
  ],
  threads: false,
})

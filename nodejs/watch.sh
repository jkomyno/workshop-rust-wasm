#!/bin/bash

set -e

GLOB=${1:-""}

# Run TypeScript tests using the compiled WebAssembly modules
pnpm test -- "${GLOB}"

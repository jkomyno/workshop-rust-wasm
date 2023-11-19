#!/bin/sh

OUT_FOLDER="../nodejs/wasm"
IN_FOLDER="./target/wasm32-unknown-unknown/release"

GLOB=${1:-""}

if [ -n "$GLOB" ]; then
    RUST_CRATE_PATTERN="w${GLOB}-*"
    WASM_LIBRARY_PATTERN="w${GLOB}_*.wasm"
else
    RUST_CRATE_PATTERN="*"
    WASM_LIBRARY_PATTERN="*.wasm"
fi

mkdir -p ${OUT_FOLDER}

# Compile Rust code to WebAssembly
echo "Compiling Rust crates matching pattern \"${RUST_CRATE_PATTERN}\"\n"
cargo build -p "${RUST_CRATE_PATTERN}" --release --target wasm32-unknown-unknown

# Generate JavaScript/TypeScript bindings from the WebAssembly artifact
for filepath in $(find ${IN_FOLDER} -maxdepth 1 -name ${WASM_LIBRARY_PATTERN}); do
  echo ""

  ext=".wasm"
  filepathroot="$(dirname ${filepath})"
  filename="$(basename ${filepath} ${ext})"
  cratename=$(echo "${filename#w}" | tr '_' '-')

  cp ${filepathroot}/${filename}${ext} ${filepathroot}/${cratename}${ext}

  echo "Running wasm-bindgen on: ${filepath}"

  outpath=${OUT_FOLDER}/${cratename}

  wasm-bindgen \
    --target bundler \
    --out-dir ${outpath} \
    --reference-types \
    ${filepathroot}/${cratename}${ext}

  echo "{\n  \"type\": \"module\"\n}" > ${outpath}/package.json

  rm ${filepathroot}/${cratename}${ext}

  echo "Generated bindings: $outpath"
done

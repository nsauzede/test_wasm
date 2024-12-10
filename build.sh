cargo build --target wasm32-unknown-unknown --release
[ -x "$(command -v wasm-gc)" ] && wasm-gc target/wasm32-unknown-unknown/release/wasm_sample.wasm
cp target/wasm32-unknown-unknown/release/wasm_sample.wasm wasm_sample.wasm

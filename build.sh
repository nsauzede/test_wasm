cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/wasm_sample.wasm
cp target/wasm32-unknown-unknown/release/wasm_sample.wasm wasm_sample.wasm

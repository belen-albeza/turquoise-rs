cargo build --release --target wasm32-unknown-unknown && \
wasm-bindgen target/wasm32-unknown-unknown/release/turquoise_rs.wasm --out-dir dist --target web --no-typescript && \
cp -r html/* dist/

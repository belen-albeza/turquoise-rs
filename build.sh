cargo build --release --target wasm32-unknown-unknown && \
wasm-bindgen target/wasm32-unknown-unknown/release/turquoise_rs.wasm --out-dir docs --target web --no-typescript && \
cp -r html/* docs/ && \
mkdir -p docs/roms && cp roms/*.rom docs/roms/

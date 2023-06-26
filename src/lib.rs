mod wasm;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(container_id: &str) -> Result<(), JsValue> {
    wasm::run(container_id);
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

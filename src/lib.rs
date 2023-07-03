mod cpu;
mod vm;
mod wasm;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(container_id: &str) -> Result<(), JsValue> {
    let cpu = cpu::CPU::new();
    let mut vm = vm::VM::new(container_id, cpu);

    vm.run().unwrap();

    Ok(())
}

#[wasm_bindgen(js_name=loadRom)]
pub fn load_rom(rom: js_sys::Uint8Array) -> Result<(), JsValue> {
    let data = rom.to_vec();
    for byte in data.into_iter() {
        web_sys::console::log_1(&format!("{}", byte).into());
    }

    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

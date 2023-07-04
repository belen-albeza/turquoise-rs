mod cpu;
mod vm;
mod wasm;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=loadRom)]
pub fn load_rom(canvas_id: &str, rom: js_sys::Uint8Array) -> Result<(), JsValue> {
    let cpu = cpu::CPU::new();
    let mut vm = vm::VM::new(canvas_id, cpu);

    vm.run(&rom.to_vec()).unwrap();

    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

mod cpu;
mod vm;
mod wasm;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(container_id: &str) -> Result<(), JsValue> {
    let mut cpu = cpu::CPU::new();
    let mut vm = vm::VM::new(container_id, cpu);

    vm.run().unwrap();

    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

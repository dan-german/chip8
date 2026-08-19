use crate::cpu::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator {
    cpu: CPU,
    rom_loaded: bool,
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Emulator {
            cpu: CPU::new(),
            rom_loaded: false,
        }
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.cpu.memory[0x200..0x200 + bytes.len()].copy_from_slice(bytes);
        self.rom_loaded = true;
    }

    pub fn step(&mut self) {
        if !self.rom_loaded {
            return;
        }
        self.cpu.step();
        // web_sys::console::log_1(&format!("pc: {}", self.cpu.pc).into());
    }

    pub fn tick_timers(&mut self) {
        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1
        }
        if self.cpu.sound_timer > 0 {
            self.cpu.sound_timer -= 1
        }
    }

    pub fn key_down(&mut self, key: usize) {
        self.cpu.keypad[key] = true;
    }
    pub fn key_up(&mut self, key: usize) {
        self.cpu.keypad[key] = false;
    }

    pub fn display_flat(&self) -> Vec<u8> {
        self.cpu
            .display
            .iter()
            .flat_map(|row| row.iter().map(|&pixel| pixel as u8))
            .collect()
    }
}

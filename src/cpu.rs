use crate::lib::*;
use rand;
use std::fs;

pub struct CPU {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [bool; 16],
    pub display: [[bool; PIXEL_COLS]; PIXEL_ROWS],
}

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl CPU {
    pub fn new() -> Self {
        let mut memory = [0u8; 4096];
        memory[0x50..0x050 + FONTSET.len()].copy_from_slice(&FONTSET);
        CPU {
            memory,
            registers: [0; 16],
            keypad: [false; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; PIXEL_COLS]; PIXEL_ROWS],
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), String> {
        let rom_bytes = fs::read(path).map_err(|e| e.to_string())?;
        if 0x200 + rom_bytes.len() > self.memory.len() {
            return Err("ROM too large to fit in memory".to_string());
        }
        self.memory[0x200..0x200 + rom_bytes.len()].copy_from_slice(&rom_bytes);
        Ok(())
    }

    pub fn fetch(&self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[self.pc as usize + 1] as u16;
        (hi << 8) | lo
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        println!("{:X}\n", opcode);
        self.pc += 2;
        self.execute(opcode);
    }

    fn op_0(&mut self, opcode: u16, nnn: u16) {
        match opcode {
            0x00E0 => {
                self.display = [[false; PIXEL_COLS]; PIXEL_ROWS];
            }
            0x00EE => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            _ => self.pc = nnn,
        }
    }

    fn apply_on_xv_with_flag(&mut self, x: usize, value: u8, flag: u8) {
        self.registers[x] = value;
        self.registers[0xF] = flag;
    }

    fn op_8(&mut self, x: usize, y: usize, n: u8) {
        let (vx, vy) = (self.registers[x], self.registers[y]);
        match n {
            0 => self.registers[x] = vy,
            1 => self.registers[x] = vx | vy,
            2 => self.registers[x] = vx & vy,
            3 => self.registers[x] = vx ^ vy,
            4 => {
                let sum: u16 = (vx as u16) + (vy as u16);
                self.apply_on_xv_with_flag(x, sum as u8, (sum > 0xFF) as u8);
            }
            5 => self.apply_on_xv_with_flag(x, vx.wrapping_sub(vy), (vx >= vy) as u8),
            6 => self.apply_on_xv_with_flag(x, vy >> 1, vy & 1),
            7 => self.apply_on_xv_with_flag(x, vy.wrapping_sub(vx), (vy >= vx) as u8),
            0xE => self.apply_on_xv_with_flag(x, vy << 1, (vy >> 7) & 1),
            _ => todo!("opcode {n}"),
        }
    }

    fn op_f(&mut self, x: usize, nn: u8) {
        match nn {
            0x07 => self.registers[x] = self.delay_timer,
            0x0A => {
                if let Some(i) = self.keypad.iter().position(|&key| key) {
                    self.registers[x] = i as u8;
                } else {
                    self.pc -= 2;
                }
            }
            0x15 => self.delay_timer = self.registers[x],
            0x18 => self.sound_timer = self.registers[x],
            0x1E => self.i += self.registers[x] as u16,
            0x29 => self.i = 0x50 + (self.registers[x] as u16 * 5),
            0x33 => {
                let mut number = self.registers[x];
                for offset in 0..3 {
                    self.memory[self.i as usize + (2 - offset)] = number % 10;
                    number /= 10;
                }
            }
            0x55 => {
                for i in 0..=x {
                    self.memory[self.i as usize] = self.registers[i];
                    self.i += 1;
                }
            }
            0x65 => {
                for i in 0..=x {
                    self.registers[i] = self.memory[self.i as usize];
                    self.i += 1;
                }
            }
            _ => todo!("opcode {nn}"),
        }
    }

    fn op_d(&mut self, x: usize, y: usize, n: usize) {
        let x_pos = (self.registers[x] % (PIXEL_COLS as u8)) as usize;
        let y_pos = (self.registers[y] % (PIXEL_ROWS as u8)) as usize;
        self.registers[0xF] = 0;

        for row in 0..n {
            if row + y_pos >= PIXEL_ROWS {
                break;
            }
            let sprite_byte = self.memory[self.i as usize + row];
            for col in 0..8 {
                if col + x_pos >= PIXEL_COLS {
                    break;
                }
                if (sprite_byte >> (7 - col)) & 1 == 1 {
                    let xp = x_pos + col;
                    let yp = y_pos + row;
                    if self.display[yp][xp] {
                        self.registers[0xF] = 1;
                    }
                    self.display[yp][xp] ^= true;
                }
            }
        }
    }

    pub fn cxnn(&mut self, x: usize, nn: u8, random_byte: u8) {
        self.registers[x] = random_byte & nn;
    }

    pub fn execute(&mut self, opcode: u16) {
        let op = (opcode & 0xF000) >> 12;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;
        let vx = self.registers[x];
        let vy = self.registers[y];
        match op {
            0 => self.op_0(opcode, nnn),
            1 => self.pc = nnn,
            2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            3 if vx == nn => self.pc += 2,
            4 if vx != nn => self.pc += 2,
            5 if vx == vy => self.pc += 2,
            9 if vx != vy => self.pc += 2,
            3 | 4 | 5 | 9 => {}
            6 => self.registers[x] = nn,
            7 => self.registers[x] = vx.wrapping_add(nn),
            8 => self.op_8(x, y, n),
            0xA => self.i = nnn,
            0xB => self.pc = nnn + self.registers[0] as u16,
            0xC => self.cxnn(x, nn, rand::random::<u8>()),
            0xE => self.op_e(opcode, nn, vx),
            0xD => self.op_d(x, y, n as usize),
            0xF => self.op_f(x, nn),
            _ => todo!("opcode {opcode:04X}"),
        }
    }

    fn op_e(&mut self, opcode: u16, nn: u8, vx: u8) {
        let pressed = self.keypad[vx as usize & 0xF];
        match nn {
            0x9E if pressed => self.pc += 2,
            0xA1 if !pressed => self.pc += 2,
            0x9E | 0xA1 => {}
            _ => todo!("opcode {opcode:04X}"),
        }
    }

    pub fn print_display(&self) {
        for row in 0..PIXEL_ROWS {
            for col in 0..PIXEL_COLS {
                print!("{}", if self.display[row][col] { "X" } else { "_" });
            }
            println!()
        }
        println!()
    }
}
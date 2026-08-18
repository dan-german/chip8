use chip8::lib::*;

fn main() {
    let mut cpu = CPU::new();
    cpu.load_rom("IBM Logo.ch8").unwrap();
    for _ in 0..30 { 
        cpu.step();
    }
    cpu.print_display();
}

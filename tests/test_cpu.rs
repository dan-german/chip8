use chip8::cpu::*;

fn write_op(cpu: &mut CPU, opcode: u16) {
    let [hi, lo] = opcode.to_be_bytes();
    cpu.memory[cpu.pc as usize] = hi;
    cpu.memory[(cpu.pc + 1) as usize] = lo;
}

fn write_op_and_step(cpu: &mut CPU, opcode: u16) {
    write_op(cpu, opcode);
    cpu.step();
}

fn cpu_after(opcode: u16) -> CPU {
    let mut cpu = CPU::new();
    write_op_and_step(&mut cpu, opcode);
    cpu
}

#[test]
fn test_fetch() {
    let mut cpu = CPU::new();
    let op = 0x6322;
    write_op(&mut cpu, op);
    assert_eq!(cpu.fetch(), op);
}

#[test]
fn test_6xnn() {
    let cpu = cpu_after(0x6322);
    assert_eq!(cpu.registers[3], 0x22);
    assert_eq!(cpu.pc, 0x202);
}

#[test]
fn test_7xnn() {
    let mut cpu = cpu_after(0x6009);
    write_op_and_step(&mut cpu, 0x7001);
    assert_eq!(cpu.registers[0], 0x0A)
}

#[test]
fn test_1nnn() {
    assert_eq!(cpu_after(0x1CCC).pc, 0x0CCC);
}

fn assert_skip(regs: &[(usize, u8)], opcode: u16, skip: bool) {
    let mut cpu = CPU::new();
    let pc = cpu.pc;
    for &(i, val) in regs {
        cpu.registers[i] = val;
    }
    write_op_and_step(&mut cpu, opcode);
    let offset = if skip { 4 } else { 2 };
    assert_eq!(cpu.pc, pc + offset);
}

#[test]
fn op_3xnn() {
    assert_skip(&[(9, 0x88)], 0x3988, true);
    assert_skip(&[(9, 0x00)], 0x3988, false);
}

#[test]
fn op_4xnn() {
    assert_skip(&[(11, 0x10)], 0x4B11, true);
    assert_skip(&[(0, 0x11)], 0x4011, false);
}

#[test]
fn op_5xy0() {
    assert_skip(&[(0, 5), (1, 5)], 0x5010, true);
    assert_skip(&[(1, 5), (2, 10)], 0x5120, false);
}

#[test]
fn op_9xy0() {
    assert_skip(&[(0, 5), (1, 5)], 0x9010, false);
    assert_skip(&[(1, 5), (2, 10)], 0x9120, true);
}

#[test]
fn op_2nnn() {
    let mut cpu = CPU::new();
    let prev_pc = cpu.pc + 2;
    write_op_and_step(&mut cpu, 0x2300);
    assert_eq!(cpu.stack[(cpu.sp - 1) as usize], prev_pc);
    assert_eq!(cpu.pc, 0x0300);
    assert_eq!(cpu.sp, 1);
}

#[test]
fn op_00ee() {
    let mut cpu = CPU::new();
    let prev_pc = cpu.pc + 2;
    write_op_and_step(&mut cpu, 0x2300);
    assert_eq!(cpu.sp, 1);
    write_op_and_step(&mut cpu, 0x00EE);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.pc, prev_pc);
}

fn test_8_op(n: u16, xv: u8, xy: u8, expected_xv: u8, expected_f: u8) {
    let mut cpu = CPU::new();
    let opcode: u16 = 0x8010 | n;
    cpu.registers[0] = xv;
    cpu.registers[1] = xy;
    write_op_and_step(&mut cpu, opcode);
    assert_eq!(cpu.registers[0], expected_xv);
    assert_eq!(cpu.registers[0xF], expected_f);
}

#[test]
fn op_8xy0() {
    test_8_op(0, 0xA, 0xB, 0xB, 0)
}

#[test]
fn op_8xy1() {
    test_8_op(1, 0b10, 0b01, 0b11, 0)
}

#[test]
fn op_8xy2() {
    test_8_op(2, 0b1011, 0b1101, 0b1001, 0)
}

#[test]
fn op_8xy3() {
    test_8_op(3, 0b1010, 0b1100, 0b0110, 0)
}

#[test]
fn op_8xy4() {
    test_8_op(4, 0b10, 0b10, 0b100, 0);
    test_8_op(4, 0xFF, 0x01, 0, 1);
}

#[test]
fn op_8xy5() {
    test_8_op(5, 0xFF, 0x01, 0xFE, 1);
    test_8_op(5, 0x00, 0x01, 0xFF, 0);
}

#[test]
fn op_8xy6() {
    test_8_op(6, 0b00, 0b10, 0b01, 0);
    test_8_op(6, 0b00, 0b11, 0b01, 1);
}

#[test]
fn op_8xy7() {
    test_8_op(7, 4, 10, 6, 1);
    test_8_op(7, 2, 1, 255, 0);
}

#[test]
fn op_8xye() {
    test_8_op(0xE, 100, 0b1000_0000, 0b0000_0000, 1);
    test_8_op(0xE, 5, 0b0000_0001, 0b0000_0010, 0);
}

#[test]
fn test_annn() {
    assert_eq!(cpu_after(0xAABC).i, 0x0ABC);
}

#[test]
fn test_bnnn() {
    assert_eq!(cpu_after(0xBABC).pc, 0xABC);
}

#[test]
fn test_cnnn() {
    let mut cpu = CPU::new();
    let mock_rand = 0b1101_0110;
    let mask = 0x0F;
    cpu.cxnn(0, mask, mock_rand);
    assert_eq!(cpu.registers[0], mock_rand & mask);
}

#[test]
fn test_0xe() {
    fn test(opcode: u16, expected_pc_inc: u16, pressed: bool) {
        let mut cpu = CPU::new();
        let pc = cpu.pc;
        cpu.keypad[15] = pressed;
        cpu.registers[0] = 255;
        write_op_and_step(&mut cpu, opcode);
        assert_eq!(cpu.pc, pc + expected_pc_inc);
    }
    test(0xE09E, 4, true);
    test(0xE09E, 2, false);
    test(0xE0A1, 4, false);
    test(0xE0A1, 2, true);
}

#[test]
fn test_fx07() {
    let mut cpu = CPU::new();
    cpu.delay_timer = 3;
    cpu.registers[0xA] = 10;
    write_op_and_step(&mut cpu, 0xFA07);
    assert_eq!(cpu.registers[0xA], 3);
}

fn bits_to_bool_vec(input: u16, n: usize) -> Vec<bool> {
    (0..n).map(|i| (input >> (n - 1 - i)) & 1 == 1).collect()
}

#[test]
fn test_dxyn() {
    let mut cpu = CPU::new();
    let sprite = 0b1010_0101;
    let sprite2 = 0b1111_0000;
    cpu.memory[0x210] = sprite;
    cpu.memory[0x211] = sprite2;
    cpu.i = 0x210;
    cpu.display = [[false; PIXEL_COLS]; PIXEL_ROWS];
    cpu.registers[0] = 0;
    cpu.registers[1] = 0;
    write_op_and_step(&mut cpu, 0xD012);
    assert_eq!(cpu.display[0][0..8], bits_to_bool_vec(sprite as u16, 8));
    assert_eq!(cpu.display[1][0..8], bits_to_bool_vec(sprite2 as u16, 8));
    assert_eq!(cpu.registers[0xF], 0);
    let sprite3 = 0b1111_1111;
    cpu.memory[0x212] = sprite3;
    cpu.i = 0x212;
    cpu.registers[0] = 3;
    cpu.registers[1] = 1;
    write_op_and_step(&mut cpu, 0xD011);
    assert_eq!(cpu.registers[0xF], 1);
    assert_eq!(cpu.display[1][0..11], bits_to_bool_vec(0b1110_1111_111, 11));
}

#[test]
fn test_00e0() {
    let mut cpu = CPU::new();
    cpu.display = [[true; PIXEL_COLS]; PIXEL_ROWS];
    write_op_and_step(&mut cpu, 0x00E0);
    assert_eq!(cpu.display, [[false; PIXEL_COLS]; PIXEL_ROWS])
}

#[test]
fn test_fx0a() {
    let mut cpu = CPU::new();
    let pc = cpu.pc;
    for _ in 0..10 {
        write_op_and_step(&mut cpu, 0xF00A);
    }
    assert_eq!(cpu.pc, pc);
    cpu.keypad[3] = true;
    write_op_and_step(&mut cpu, 0xF00A);
    assert_eq!(cpu.pc, pc + 2);
    assert_eq!(cpu.registers[0], 3);
}

#[test]
fn test_fx15() {
    let mut cpu = CPU::new();
    cpu.delay_timer = 1;
    cpu.registers[0xA] = 9;
    write_op_and_step(&mut cpu, 0xFA15);
    assert_eq!(cpu.delay_timer, 9);
}

#[test]
fn test_fx18() {
    let mut cpu = CPU::new();
    cpu.sound_timer = 1;
    cpu.registers[0xB] = 10;
    write_op_and_step(&mut cpu, 0xFB18);
    assert_eq!(cpu.sound_timer, 10);
}

#[test]
fn test_fx1e() {
    let mut cpu = CPU::new();
    cpu.i = 1;
    cpu.registers[0] = 10;
    write_op_and_step(&mut cpu, 0xF01E);
    assert_eq!(cpu.i, 11);
}

#[test]
fn test_fx29() {
    let mut cpu = CPU::new();
    cpu.i = 1;
    cpu.registers[0] = 0;
    write_op_and_step(&mut cpu, 0xF029);
    assert_eq!(cpu.i, 0x50);

    cpu.registers[1] = 2;
    write_op_and_step(&mut cpu, 0xF129);
    assert_eq!(cpu.i, 0x50 + 10);
}

#[test]
fn test_fx33() {
    let mut cpu = CPU::new();
    cpu.i = 0x300;

    cpu.registers[0] = 255;
    write_op_and_step(&mut cpu, 0xF033);
    assert_eq!(cpu.memory[cpu.i as usize], 2);
    assert_eq!(cpu.memory[(cpu.i + 1) as usize], 5);
    assert_eq!(cpu.memory[(cpu.i + 2) as usize], 5);

    cpu.registers[0] = 34;
    write_op_and_step(&mut cpu, 0xF033);
    assert_eq!(cpu.memory[cpu.i as usize], 0);
    assert_eq!(cpu.memory[(cpu.i + 1) as usize], 3);
    assert_eq!(cpu.memory[(cpu.i + 2) as usize], 4);
}

#[test]
fn test_fx55() {
    let mut cpu = CPU::new();
    cpu.registers[0] = 0xA;
    cpu.registers[1] = 0xB;
    cpu.registers[2] = 0xC;
    cpu.i = 0x250;

    assert_eq!(cpu.memory[0x250], 0u8);
    assert_eq!(cpu.memory[0x251], 0u8);
    assert_eq!(cpu.memory[0x252], 0u8);

    write_op_and_step(&mut cpu, 0xF255);

    assert_eq!(cpu.memory[0x250], 0xA);
    assert_eq!(cpu.memory[0x251], 0xB);
    assert_eq!(cpu.memory[0x252], 0xC);
    assert_eq!(cpu.i, 0x253);
}

#[test]
fn test_fx65() {
    let mut cpu = CPU::new();
    cpu.memory[0x290] = 0xE;
    cpu.memory[0x291] = 0xF;
    cpu.memory[0x292] = 0x1;
    cpu.memory[0x293] = 0x5;
    cpu.i = 0x290;
    cpu.registers[4] = 8;

    write_op_and_step(&mut cpu, 0xF365);

    assert_eq!(cpu.registers[0], 0xE);
    assert_eq!(cpu.registers[1], 0xF);
    assert_eq!(cpu.registers[2], 0x1);
    assert_eq!(cpu.registers[3], 0x5);
    assert_eq!(cpu.registers[4], 8);
    assert_eq!(cpu.i, 0x294);
}
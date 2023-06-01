use std::println;

use chip_8_toy::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    // 8 - operation involves two registers
    // 0 - maps to cpu.registers[0]
    // 1 - maps to cpu.registers[1]
    // 4 - indicates addition
    mem[512] = 0x80;
    mem[513] = 0x14;

    mem[514] = 0x80;
    mem[515] = 0x24;

    mem[516] = 0x80;
    mem[517] = 0x34;

    cpu.run();

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);

    cpu.reset();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    // Add twice function at address 0x400
    mem[0x400] = 0x80;
    mem[0x401] = 0x14;
    mem[0x402] = 0x80;
    mem[0x403] = 0x14;
    mem[0x404] = 0x00;
    mem[0x405] = 0xEE;

    // Call function twice
    mem[0x200] = 0x24;
    mem[0x201] = 0x00;
    mem[0x202] = 0x24;
    mem[0x203] = 0x00;
    // End program
    mem[0x204] = 0x00;
    mem[0x205] = 0x00;

    cpu.run();

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

use std::println;

use chip_8_toy::CPU;

fn main() {
    let mut cpu = CPU {
        program_counter: 0,
        registers: [0; 16],
        memory: [0; 4096],
    };

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
}

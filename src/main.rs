use std::println;

use chip_8_toy::CPU;

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    // 8 - operation involves two registers
    // 0 - maps to cpu.registers[0]
    // 1 - maps to cpu.registers[1]
    // 4 - indicates addition
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    println!("5 + 10 = {}", cpu.registers[0]);
}

use core::panic;
use std::todo;

pub struct CPU {
    program_counter: usize,
    pub registers: [u8; 16],
    pub memory: [u8; 0x1000],
    stack: [u16; 32],
    stack_pointer: usize,
}

impl CPU {
    const SYSTEM_OFFEST: u16 = 0x200;

    pub fn new() -> Self {
        CPU {
            program_counter: Self::SYSTEM_OFFEST as usize,
            registers: [0; 16],
            memory: [0; 4096],
            stack: [0; 32],
            stack_pointer: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        let op_byte1 = self.memory[self.program_counter] as u16;
        let op_byte2 = self.memory[self.program_counter + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();

            let op_subgroup = (opcode & 0x000F) as u8;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;

            let nnn = opcode & 0x0FFF;

            self.program_counter += 2;

            match opcode {
                0x0000 => return,
                0x00EE => self.ret(),
                0x2000..=0x2FFF => self.call(nnn),
                0x8000..=0x8FFF => match op_subgroup {
                    4 => self.add_xy(x, y),
                    _ => todo!("opcode {:04x}", opcode),
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        let (val, is_overflown) = x_.overflowing_add(y_);
        self.registers[x as usize] = val;

        if is_overflown {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        if addr < Self::SYSTEM_OFFEST {
            panic!("Trying to access memory reserved for the system!")
        }

        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stach overflow!")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn addition() {
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

        assert_eq!(cpu.registers[0], 35);
    }

    #[test]
    fn call_and_return() {
        let mut cpu = CPU::new();

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

        assert_eq!(cpu.registers[0], 45);
    }
}
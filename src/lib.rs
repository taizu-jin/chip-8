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

            let kk = (opcode & 0x00FF) as u8;
            let nnn = opcode & 0x0FFF;

            self.program_counter += 2;

            match opcode {
                0x0000 => return,
                0x00EE => self.ret(),
                0x1000..=0x1FFF => self.jmp(nnn),
                0x2000..=0x2FFF => self.call(nnn),
                0x3000..=0x3FFF => self.se(x, kk),
                0x4000..=0x4FFF => self.sne(x, kk),
                0x5000..=0x5FFF => self.se(x, y),
                0x6000..=0x6FFF => self.ld(x, kk),
                0x7000..=0x7FFF => self.add(x, kk),
                0x8000..=0x8FFF => match op_subgroup {
                    0 => self.ld(x, self.registers[y as usize]),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => self.add(x, self.registers[y as usize]),
                    5 => self.sub_xy(x, y),
                    6 => self.shr_xy(x, y),
                    7 => self.subn_xy(x, y),
                    8 => self.shl_xy(x, y),
                    _ => todo!("opcode {:04x}", opcode),
                },
                0x9000..=0x9FF0 => self.sne(x, self.registers[y as usize]),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
    /// 8xy6 - SHL Vx {, Vy}.
    ///
    /// Set Vx = Vx SHL 1. If the most-significant bit of Vx is 1, then VF set to 1, otherwose 0.
    /// then Vx is multiplied by 2.
    fn shl_xy(&mut self, vx: u8, _vy: u8) {
        let x = self.registers[vx as usize];
        self.registers[0xF] = (x & 0x80) >> 7;
        self.registers[vx as usize] = x << 1;
    }

    /// 8xy7 - subn Vx, Vy.
    ///
    /// Set Vx = Vy - Vx, set VF = NOT Borrow. If Vy ¿ Vx, then VF i set to 1, otherwise 0. Then Vx
    /// is subtracted from Vy, and the results stored in Vx.
    fn subn_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        let (val, is_overflown) = y.overflowing_sub(x);

        self.registers[vx as usize] = val;

        if is_overflown {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    /// 8xy6 - SHR Vx {, Vy}.
    ///
    /// Set Vx = Vx SHR 1. If the least-significant bit of Vx is 1, then VF set to 1, otherwose 0.
    /// then Vx is divided by 2.
    fn shr_xy(&mut self, vx: u8, _vy: u8) {
        let x = self.registers[vx as usize];
        self.registers[0xF] = x & 0x01;
        self.registers[vx as usize] = x >> 1;
    }

    /// 8xy5 - SUB Vx, Vy.
    ///
    /// Set Vx = Vx - Vy, set VF = NOT borrow. if Vx ¿ Vy, then VF is set to 1, otherwise 0. Then
    /// Vy is subtrated from Vx, and the results stored in Vx.
    fn sub_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        let (val, is_overflown) = x.overflowing_sub(y);

        self.registers[vx as usize] = val;

        if is_overflown {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    /// 8xy1 - XOR Vx, Vy.
    ///
    /// Set Vx = Vx XOR Vy. Performs  a bitwise exclusive OR on the values of Vx and Vy, then stores the
    /// result in Vx. An exclusive bitwise OR compares bits from two values, and if the bits arne
    /// not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is
    /// 0.
    fn xor_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[vx as usize] = x ^ y;
    }

    /// 8xy1 - OR Vx, Vy.
    ///
    /// Set Vx = Vx OR Vy. Performs  a bitwise OR on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise OR compares the corresponding bits from two values, and if either
    /// bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    fn or_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[vx as usize] = x | y;
    }

    /// 8xy2 - AND Vx, Vy.
    ///
    /// Set Vx = Vx AND Vy. Performs  a bitwise AND on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise AND compares the corresponding bits from two values, and if both
    /// bits are 1,then the same bit in the result is also 1. Otherwise, it is 0.
    fn and_xy(&mut self, vx: u8, vy: u8) {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[vx as usize] = x & y;
    }

    /// 1nnn - JP addr.
    ///
    /// Jump to location nnn. The interpreter sets the program counter to nnn.
    fn jmp(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }

    /// 3xkk - SE Vx, byte.
    ///
    /// Skip next instructon if Vx = kk. The interpreter compares register Vx to kk, and if they
    /// are equal, increments the program counter by 2.
    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.program_counter += 2;
        }
    }

    /// 4xkk - SNE Vx, byte.
    ///
    /// Skip next instruction if Vx != kk. The interpreter compares register Vx to kk, and if they
    /// are not equal, increments the program counter by 2.
    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.program_counter += 2;
        }
    }

    /// 6xkk - LD Vx, byte.
    ///
    /// Set Vx = kk. The interpreter puts the value kk into register Vx.
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    /// 7xkk - ADD Vx, byte.
    ///
    /// Set Vx = Vx + kk. Adds the value kk to the value of register Vx, then stores the result in
    /// Vx.
    fn add(&mut self, vx: u8, kk: u8) {
        let x = self.registers[vx as usize];
        let (val, is_overflown) = x.overflowing_add(kk);

        self.registers[vx as usize] = val;

        if is_overflown {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    /// 2nnn - CALL addr.
    ///
    /// Call subroutine at nnn. The interpreter increments the stack pointer, then puts the current
    /// PC on the top of the stack. The PC is then set to nnn.
    ///
    /// # Panics
    ///
    /// 1. Address is in reserved system memory (until 0x200).
    /// 2. Stack overflow.
    fn call(&mut self, addr: u16) {
        if addr < Self::SYSTEM_OFFEST {
            panic!("Trying to call from memory reserved for the system!")
        }

        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    /// 00EE - RET.
    ///
    /// Return from a subroutine. The interpreter sets the program counter to the address at the
    /// top of the stack, then substracts 1 from the stack pointer.
    ///
    /// # Panics
    ///
    /// Stack underflow.
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
    }

    /// Reset CPU to initial state.
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

    #[test]
    fn shr_xy() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 5;

        let mem = &mut cpu.memory;

        mem[0x200] = 0x80;
        mem[0x201] = 0x16;

        cpu.run();

        assert_eq!(2, cpu.registers[0]);
        assert_eq!(1, cpu.registers[0xF]);

        cpu.reset();
        cpu.registers[0] = 4;

        let mem = &mut cpu.memory;

        mem[0x200] = 0x80;
        mem[0x201] = 0x16;

        cpu.run();

        assert_eq!(2, cpu.registers[0]);
        assert_eq!(0, cpu.registers[0xF]);
    }

    #[test]
    fn shl_xy() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 0x81;

        let mem = &mut cpu.memory;

        mem[0x200] = 0x80;
        mem[0x201] = 0x18;

        cpu.run();

        assert_eq!(2, cpu.registers[0]);
        assert_eq!(1, cpu.registers[0xF]);

        cpu.reset();
        cpu.registers[0] = 0x01;

        let mem = &mut cpu.memory;

        mem[0x200] = 0x80;
        mem[0x201] = 0x18;

        cpu.run();

        assert_eq!(2, cpu.registers[0]);
        assert_eq!(0, cpu.registers[0xF]);
    }
}

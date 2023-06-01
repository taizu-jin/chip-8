use std::todo;

pub struct CPU {
    pub program_counter: usize,
    pub registers: [u8; 16],
    pub memory: [u8; 0x1000],
}

impl CPU {
    const SYSTEM_OFFEST: u16 = 0x200;

    fn read_opcode(&self) -> u16 {
        let p = self.program_counter + Self::SYSTEM_OFFEST as usize;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn addition() {
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

        assert_eq!(cpu.registers[0], 35);
    }
}

use super::cpu::{Chip8, Nibbles};

impl Chip8 {
    // The interpreter sets the program counter to the address at the top of the stack,
    // then subtracts 1 from the stack pointer.
    fn exec_00e0(&mut self) {
        self.pc = self.stack[self.sp] as usize;
        self.sp -= 1;
    }

    // Jump to location nnn.
    //The interpreter sets the program counter to nnn.
    fn exec_1nnn(&mut self, nnn: &usize) {
        self.pc = *nnn;
    }

    // The interpreter increments the stack pointer,
    // then puts the current PC on the top of the stack.
    // The PC is then set to nnn.
    fn exec_2nnn(&mut self, nnn: &usize) {
        self.sp += 1;
        self.stack[self.sp] = self.pc as u16;
        self.pc = *nnn;
    }

    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn exec_3xkk(&self, x: u8, kk: u8) -> bool {
        self.v_registers[x as usize] == kk
    }

    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn exec_4xkk(&self, x: u8, kk: u8) -> bool {
        self.v_registers[x as usize] != kk
    }

    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn exec_5xy0(&self, x: u8, y: u8) -> bool {
        self.v_registers[x as usize] == self.v_registers[y as usize]
    }

    // The interpreter puts the value kk into register Vx.
    fn exec_6xkk(&mut self, x: u8, kk: u8) {
        self.v_registers[x as usize] = kk
    }

    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn exec_7xkk(&mut self, x: u8, kk: u8) {
        let vx_val = self.v_registers[x as usize] as u16;
        let val = kk as u16;
        let result = vx_val + val;
        self.v_registers[x as usize] = result as u8;
    }

    // Stores the value of register Vy in register Vx.
    fn exec_8xy0(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] = self.v_registers[y as usize];
    }

    // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    fn exec_8xy1(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] |= self.v_registers[y as usize];
    }

    // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    fn exec_8xy2(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] &= self.v_registers[y as usize];
    }

    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    fn exec_8xy3(&mut self, x: u8, y: u8) {
        self.v_registers[x as usize] ^= self.v_registers[y as usize];
    }

    // The values of Vx and Vy are added together.
    // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn exec_8xy4(&mut self, x: u8, y: u8) {
        let v_x = self.v_registers[x as usize] as u16;
        let v_y = self.v_registers[y as usize] as u16;
        let result = v_x + v_y;
        self.v_registers[x as usize] = result as u8;
        if result > 0xFF {
            self.v_registers[0x0F] = 1;
        } else {
            self.v_registers[0x0F] = 0;
        }
    }

    // Set Vx = Vx - Vy, set VF = NOT borrow.
    // If Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    fn exec_8xy5(&mut self, x: u8, y: u8) {
        let v_x = self.v_registers[x as usize];
        let v_y = self.v_registers[y as usize];
        if v_x > v_y {
            self.v_registers[0xF] = 1;
        } else {
            self.v_registers[0xF] = 0;
        }
        self.v_registers[x as usize] =
            self.v_registers[x as usize].wrapping_sub(self.v_registers[y as usize]);
    }

    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn exec_8xy6(&mut self, x: u8) {
        self.v_registers[0x0F] = self.v_registers[x as usize] & 1;
        self.v_registers[x as usize] = self.v_registers[x as usize] >> 1;
    }
}

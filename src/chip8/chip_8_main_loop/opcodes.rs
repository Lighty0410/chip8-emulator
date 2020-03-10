use crate::chip8::Chip8;
use rand::prelude::*;

impl Chip8 {
    // CLS
    pub(super) fn exec_00e0(&mut self) {
        for height in 0..32 {
            for width in 0..64 {
                self.resources.gfx[width][height] = 0;
            }
        }
        self.fontset_is_changed = true;
    }

    // The interpreter sets the program counter to the address at the top of the stack,
    // then subtracts 1 from the stack pointer.
    pub(super) fn exec_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }

    // Jump to location nnn.
    //The interpreter sets the program counter to nnn.
    pub(super) fn exec_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }

    // The interpreter increments the stack pointer,
    // then puts the current PC on the top of the stack.
    // The PC is then set to nnn.
    pub(super) fn exec_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp] = (self.pc + 2) as u16;
        self.sp += 1;
        self.pc = nnn;
    }

    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    pub(super) fn exec_3xkk(&mut self, x: usize, kk: u8) {
        if self.v_registers[x] == kk {
            self.pc += 2;
        };
    }

    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    pub(super) fn exec_4xkk(&mut self, x: usize, kk: u8) {
        if self.v_registers[x] != kk {
            self.pc += 2;
        };
    }

    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    pub(super) fn exec_5xy0(&mut self, x: usize, y: usize) {
        if self.v_registers[x] == self.v_registers[y] {
            self.pc += 2
        }
    }

    // The interpreter puts the value kk into register Vx.
    pub(super) fn exec_6xkk(&mut self, x: usize, kk: u8) {
        self.v_registers[x] = kk;
    }

    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    pub(super) fn exec_7xkk(&mut self, x: usize, kk: u8) {
        let vx_val = self.v_registers[x] as u16;
        let val = kk as u16;
        let result = vx_val + val;
        self.v_registers[x] = result as u8;
    }

    // Stores the value of register Vy in register Vx.
    pub(super) fn exec_8xy0(&mut self, x: usize, y: usize) {
        self.v_registers[x] = self.v_registers[y];
    }

    // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    pub(super) fn exec_8xy1(&mut self, x: usize, y: usize) {
        self.v_registers[x] |= self.v_registers[y];
    }

    // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    pub(super) fn exec_8xy2(&mut self, x: usize, y: usize) {
        self.v_registers[x] &= self.v_registers[y];
    }

    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    pub(super) fn exec_8xy3(&mut self, x: usize, y: usize) {
        self.v_registers[x] ^= self.v_registers[y];
    }

    // The values of Vx and Vy are added together.
    // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept, and stored in Vx.
    pub(super) fn exec_8xy4(&mut self, x: usize, y: usize) {
        let v_x = self.v_registers[x] as u16;
        let v_y = self.v_registers[y] as u16;
        let result = v_x + v_y;
        self.v_registers[x] = result as u8;
        if result > 0xFF {
            self.v_registers[0x0F] = 1;
        } else {
            self.v_registers[0x0F] = 0;
        }
    }

    // Set Vx = Vx - Vy, set VF = NOT borrow.
    // If Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    pub(super) fn exec_8xy5(&mut self, x: usize, y: usize) {
        let v_x = self.v_registers[x];
        let v_y = self.v_registers[y];
        if v_x > v_y {
            self.v_registers[0x0F] = 1;
        } else {
            self.v_registers[0x0F] = 0;
        }
        self.v_registers[x] = self.v_registers[x].wrapping_sub(self.v_registers[y]);
    }

    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    pub(super) fn exec_8xy6(&mut self, x: usize) {
        self.v_registers[0x0F] = self.v_registers[x] & 1;
        self.v_registers[x] >>= 1;
    }

    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    pub(super) fn exec_8xy7(&mut self, x: usize, y: usize) {
        let v_x = self.v_registers[x];
        let v_y = self.v_registers[y];
        if v_y > v_x {
            self.v_registers[0x0F] = 1;
        } else {
            self.v_registers[0x0F] = 0;
        }
        self.v_registers[x] = self.v_registers[y].wrapping_sub(self.v_registers[x]);
    }

    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    pub(super) fn exec_8xye(&mut self, x: usize) {
        self.v_registers[0x0F] = (self.v_registers[x] & 0b1000_0000) >> 7;
        self.v_registers[x] <<= 1;
        // let mut val = self.v_registers[x] as u8;
        // val &= 0xF0;
        // val >>= 7;
        // if val == 1 {
        //     self.v_registers[0x0F] = 1;
        // } else {
        //     self.v_registers[0x0F] = 0;
        // }
        // self.v_registers[x] = self.v_registers[x] << 1;
    }

    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    pub(super) fn exec_9xy0(&mut self, x: usize, y: usize) {
        if self.v_registers[x] != self.v_registers[y] {
            self.pc += 2;
        }
    }

    // The value of register I is set to nnn.
    pub(super) fn exec_annn(&mut self, nnn: u16) {
        self.i_reg = nnn;
    }

    // The program counter is set to nnn plus the value of V0.
    pub(super) fn exec_bnnn(&mut self, nnn: u16) {
        let val = nnn + self.v_registers[0x00] as u16;
        self.pc = val as usize;
    }

    // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
    // The results are stored in Vx.
    pub(super) fn exec_cxkk(&mut self, x: usize, kk: u8) {
        let mut rand_num: u8 = random();
        rand_num &= kk;
        self.v_registers[x] &= rand_num;
    }

    // The interpreter reads n bytes from memory, starting at the address stored in I.
    // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    // Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    // If the sprite is positioned so part of it is outside the coordinates of the display,
    // it wraps around to the opposite side of the screen.
    // This kicks my ass a lil' bit.
    // So i used this solution. Thanks a lot!
    // https://github.com/starrhorne/chip8-rust/blob/345602a97288fd8d69dafd6684e8f51cd38e95e2/src/processor.rs#L340
    // TODO: rewrite in more understandable way.
    pub(super) fn exec_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v_registers[0x0F] = 0;
        for byte in 0..n {
            let y = (self.v_registers[y] as usize + byte) % 32;
            for bit in 0..8 {
                let x = (self.v_registers[x] as usize + bit) % 64;
                let color = (self.memory[self.i_reg as usize + byte] >> (7 - bit)) & 1;
                self.v_registers[0x0F] |= color & self.resources.gfx[y][x];
                self.resources.gfx[y][x] ^= color;
            }
        }
        self.fontset_is_changed = true;
    }

    //Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    pub(super) fn exec_ex9e(&mut self, x: usize) {
        if self.resources.key[self.v_registers[x] as usize] {
            self.pc += 2;
        }
    }

    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    pub(super) fn exec_exa1(&mut self, x: usize) {
        if !self.resources.key[self.v_registers[x] as usize] {
            self.pc += 2;
        }
    }

    // The value of DT is placed into Vx.
    pub(super) fn exec_fx07(&mut self, x: usize) {
        self.v_registers[x] = self.delay_timer;
    }

    // Wait for a key press, store the value of the key in Vx.
    pub(super) fn exec_fx0a(&mut self, x: usize) {
        self.resources.is_key_waiting = true;
        self.v_registers[x] = self.resources.key_value;
    }

    // DT is set equal to the value of Vx.
    pub(super) fn exec_fx15(&mut self, x: usize) {
        self.delay_timer = self.v_registers[x];
    }

    // ST is set equal to the value of Vx.
    pub(super) fn exec_fx18(&mut self, x: usize) {
        self.sound_timer = self.v_registers[x];
    }

    // The values of I and Vx are added, and the results are stored in I.
    pub(super) fn exec_fx1e(&mut self, x: usize) {
        let val = self.v_registers[x] as u16 + self.i_reg;
        self.i_reg = val;
    }

    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx
    pub(super) fn exec_fx29(&mut self, x: usize) {
        self.i_reg = (self.v_registers[x] as u16) * 5;
    }

    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    //  the tens digit at location I+1, and the ones digit at location I+2.
    pub(super) fn exec_fx33(&mut self, x: usize) {
        self.memory[self.i_reg as usize] = self.v_registers[x] / 100;
        self.memory[(self.i_reg + 1) as usize] = (self.v_registers[x] & 100) / 10;
        self.memory[(self.i_reg + 2) as usize] = self.v_registers[x] % 10;
    }

    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    pub(super) fn exec_fx55(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.memory[(self.i_reg as usize) + i as usize] = self.v_registers[i as usize];
        }
    }

    //The interpreter reads values from memory starting at location I into registers V0 through Vx.
    pub(super) fn exec_fx65(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.v_registers[i as usize] = self.memory[self.i_reg as usize + i as usize];
        }
    }
}

mod opcodes;

struct Nibbles {
    first_bite: u8,
    nnn: usize,
    nibble: u8,
    x: usize,
    y: usize,
    kk: u8,
}

impl super::Chip8 {
    pub fn tick(&mut self, keyboard: [bool; 16]) -> Option<&[[u8; 64]; 32]> {
        self.resources.key = keyboard;
        self.fontset_is_changed = false;

        if self.resources.is_key_waiting {
            for (i, &valid_key) in self.resources.key.iter().enumerate() {
                if valid_key {
                    self.resources.is_key_waiting = false;
                    self.v_registers[self.resources.key_value as usize] = i as u8;
                }
            }
        } else {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
            self.fetch_opcode();
            let nib = self.decode_opcode();
            self.execute_opcode(&nib);
        }
        if !self.fontset_is_changed {
            None
        } else {
            Some(&self.resources.gfx)
        }
    }

    fn fetch_opcode(&mut self) {
        self.opcode = self.memory[self.pc] as u16;
        self.opcode <<= 8;
        self.opcode |= self.memory[self.pc + 1] as u16;
    }

    fn decode_opcode(&self) -> Nibbles {
        let raw_nibbles = (
            self.opcode & 0x0FFF, // nnn,
            self.opcode & 0x000F, // nibble,
            self.opcode & 0x0F00, // x,
            self.opcode & 0x00F0, // y,
            self.opcode & 0x00FF, // kk,
            self.opcode & 0xF000, // first bite
        );

        Nibbles {
            nnn: raw_nibbles.0 as usize,
            nibble: raw_nibbles.1 as u8,
            x: (raw_nibbles.2 >> 8) as usize,
            y: (raw_nibbles.3 >> 4) as usize,
            kk: raw_nibbles.4 as u8,
            first_bite: (raw_nibbles.5 >> 12) as u8,
        }
    }

    pub fn load_into_memory(&mut self, bytes: &[u8]) {
        for (i, &byte) in bytes.iter().enumerate() {
            if self.pc + i > 4096 {
                break;
            }
            self.memory[self.pc + i] = byte;
        }
    }

    fn execute_opcode(&mut self, nib: &Nibbles) {
        let nibbles = (nib.first_bite, nib.x, nib.y, nib.nibble);
        let mut is_jumped = false;
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.exec_00e0(),
            (0x0, 0x0, 0xE, 0xE) => {
                self.exec_00ee();
                is_jumped = true;
            }
            (0x1, _, _, _) => {
                self.exec_1nnn(nib.nnn);
                is_jumped = true;
            }
            (0x2, _, _, _) => {
                self.exec_2nnn(nib.nnn);
                is_jumped = true;
            }
            (0x3, _, _, _) => self.exec_3xkk(nib.x, nib.kk),
            (0x4, _, _, _) => self.exec_4xkk(nib.x, nib.kk),
            (0x5, _, _, 0x0) => self.exec_5xy0(nib.x, nib.y),
            (0x6, _, _, _) => self.exec_6xkk(nib.x, nib.kk),
            (0x7, _, _, _) => self.exec_7xkk(nib.x, nib.kk),
            (0x8, _, _, 0x0) => self.exec_8xy0(nib.x, nib.y),
            (0x8, _, _, 0x1) => self.exec_8xy1(nib.x, nib.y),
            (0x8, _, _, 0x2) => self.exec_8xy2(nib.x, nib.y),
            (0x8, _, _, 0x3) => self.exec_8xy3(nib.x, nib.y),
            (0x8, _, _, 0x4) => self.exec_8xy4(nib.x, nib.y),
            (0x8, _, _, 0x5) => self.exec_8xy5(nib.x, nib.y),
            // 8xy6 <- interesting. Typo in cowgod's manual or ... ? TODO: check.
            (0x8, _, _, 0x6) => self.exec_8xy6(nib.x),
            (0x8, _, _, 0x7) => self.exec_8xy7(nib.x, nib.y),
            (0x8, _, _, 0xE) => self.exec_8xye(nib.x),
            (0x9, _, _, 0x0) => self.exec_9xy0(nib.x, nib.y),
            (0xA, _, _, _) => self.exec_annn(nib.nnn as u16),
            (0xB, _, _, _) => {
                self.exec_bnnn(nib.nnn as u16);
                is_jumped = true
            }
            (0xC, _, _, _) => self.exec_cxkk(nib.x, nib.kk),
            (0xD, _, _, _) => self.exec_dxyn(nib.x, nib.y, nib.nibble as usize),
            (0xE, _, 0x9, 0xE) => self.exec_ex9e(nib.x),
            (0xE, _, 0xA, 0x1) => self.exec_exa1(nib.x),
            (0xF, _, 0x0, 0x7) => self.exec_fx07(nib.x),
            (0xF, _, 0x0, 0xA) => self.exec_fx0a(nib.x),
            (0xF, _, 0x1, 0x5) => self.exec_fx15(nib.x),
            (0xF, _, 0x1, 0x8) => self.exec_fx18(nib.x),
            (0xF, _, 0x1, 0xE) => self.exec_fx1e(nib.x),
            (0xF, _, 0x2, 0x9) => self.exec_fx29(nib.x),
            (0xF, _, 0x3, 0x3) => self.exec_fx33(nib.x),
            (0xF, _, 0x5, 0x5) => self.exec_fx55(nib.x),
            (0xF, _, 0x6, 0x5) => self.exec_fx65(nib.x),
            // skip instructions
            _ => self.pc += 2,
        }
        if !is_jumped {
            self.pc += 2;
        }
    }
}

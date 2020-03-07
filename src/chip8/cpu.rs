pub struct Chip8 {
    pub stack: [u16; 16],
    pub sp: usize,

    pub memory: [u8; 4096],
    pub v_registers: [u8; 16],

    pub pc: usize, // TODO: try to change it to the reference in the future in order to make emulator more idimatic(?).
    pub opcode: u16,
    pub i_reg: u16,

    pub delay_timer: u8,
    pub sound_timer: u8,
    resources: ExternalResources,
}

pub struct Nibbles {
    nnn: usize,
    nibble: u8,
    x: u8,
    y: u8,
    kk: u8,
}

static chip8_fontset: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80, //F
];

struct ExternalResources {
    gfx: [u8; 64 * 32],
    key: [u8; 16],
    draw_flag: bool,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            pc: 0x200,
            opcode: 0,
            i_reg: 0,
            sp: 0,
            memory: [0; 4096],
            v_registers: [0; 16],
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,

            resources: ExternalResources {
                gfx: [0; 64 * 32],
                key: [0; 16],
                draw_flag: false,
            },
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
        );

        Nibbles {
            nnn: raw_nibbles.0 as usize,
            nibble: raw_nibbles.1 as u8,
            x: (raw_nibbles.2 >> 8) as u8,
            y: (raw_nibbles.3 >> 4) as u8,
            kk: raw_nibbles.4 as u8,
        }
    }

    fn execute_opcode(&self, nib: Nibbles) {}
}

struct Chip8 {
    stack: [u16; 16],
    sp: u16,

    memory: [u8; 4096],
    v_registers: [u8; 16],

    pc: u16,
    opcode: u16,
    i_reg: u16,

    delay_timer: u8,
    sound_timer: u8,
    resources: ExternalResources,
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
        self.opcode =
            (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize] as u16)
    }

    fn decode_opcode(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {
                0x0000 => {
                    for g in self.resources.gfx.iter_mut() {
                        *g = 0;
                    }
                    self.resources.draw_flag = true;
                    self.pc += 2;
                }
                0x000E => {
                    self.sp -= 1;
                }
                _ => (),
            },
            _ => (),
        }
    }
}

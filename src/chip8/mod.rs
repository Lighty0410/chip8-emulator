pub mod chip_8_main_loop;
pub struct Chip8 {
    stack: [u16; 16],
    sp: usize,

    memory: [u8; 4096],
    v_registers: [u8; 16],

    pc: usize, // TODO: try to change it to the reference in the future in order to make emulator more idimatic(?).
    opcode: u16,
    i_reg: u16,

    fontset_is_changed: bool,

    delay_timer: u8,
    sound_timer: u8,

    resources: ExternalResources,
}

pub struct ExternalResources {
    pub gfx: [[u8; 64]; 32],
    pub key: [bool; 16],
    key_value: u8,
    is_key_waiting: bool,
    pub draw_flag: bool,
}

static CHIP8_FONTSET: [u8; 80] = [
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

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut mem = [0u8; 4096];
        for (i, &byte) in CHIP8_FONTSET.iter().enumerate() {
            mem[i] = byte;
        }
        Chip8 {
            pc: 0x200,
            opcode: 0,
            i_reg: 0,
            sp: 0,
            memory: mem,
            v_registers: [0; 16],
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            fontset_is_changed: false,
            resources: ExternalResources {
                gfx: [[0; 64]; 32],
                key: [false; 16],
                draw_flag: false,
                is_key_waiting: false,
                key_value: 0,
            },
        }
    }
}

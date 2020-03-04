struct Cpu {
    opcode: u16,
    v_registers: [u8; 16],
    mem_adr_reg: u16,
    timers: Timers,
    subroutine_calls: u16,
    counter: u8,
    memory: [u8; 4096],
}

struct Timers {
    sound_timer: u8,
    delay_timer: u8,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            opcode: 0,
            v_registers: [0; 16],
            mem_adr_reg: 0x200,
            timers: Timers {
                sound_timer: 0,
                delay_timer: 0,
            },
            subroutine_calls: 0x200,
            counter: 0,
            memory: [0; 4096],
        }
    }
}

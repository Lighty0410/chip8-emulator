use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

const SIZE_LIMIT: usize = 0xFFF - 0x200;

pub fn read_rom(file: &str) -> Result<[u8; SIZE_LIMIT], Error> {
    let mut f = File::open(file)?;
    let mut rom_file: [u8; SIZE_LIMIT] = [0; SIZE_LIMIT];

    f.read_exact(&mut rom_file)?;

    Ok(rom_file)
}

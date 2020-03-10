mod chip8;
mod external_resources;

use external_resources::rom;

use sdl2;
use std::env;
use std::thread;
use std::time::Duration;

fn gen_env() -> Option<String> {
    for (key, val) in env::vars() {
        if key == "ROM_PATH" {
            return Some(val);
        }
    }
    None
}

fn main() -> Result<(), String> {
    let cartridge_filename = gen_env().ok_or_else(|| "incorrect file path".to_string())?;

    let rom =
        rom::read_rom(&cartridge_filename).or_else(|e| Err(format!("cannot read file: {}", e)))?;

    let sdl_context = sdl2::init()?;

    let mut screen = external_resources::Screen::new(&sdl_context)?;
    let mut input = external_resources::Input::new(&sdl_context)?;

    let mut cpu = chip8::Chip8::new();
    cpu.load_into_memory(&rom);

    while let Ok(keypad) = input.poll() {
        let tick = cpu.tick(keypad);
        if let Some(val) = tick {
            screen.draw(val)?;
        }
        thread::sleep(Duration::from_millis(2));
    }

    Ok(())
}

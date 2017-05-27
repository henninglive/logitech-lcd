extern crate logitech_lcd;

use logitech_lcd::{Lcd, MONO_WIDTH, MONO_HEIGHT};

fn main() {
    let mut lcd = Lcd::init_mono("Blink").unwrap();

    let blank_screen  = std::iter::repeat(0u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>();
    let filled_screen = std::iter::repeat(255u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>();

    for i in 0..10 {
        match i % 2 == 0 {
            true  => lcd.set_mono_background(&blank_screen[..]).unwrap(),
            false => lcd.set_mono_background(&filled_screen[..]).unwrap(),
        }
        lcd.update();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
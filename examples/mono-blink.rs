extern crate logitech_lcd;

use logitech_lcd::{Lcd, LcdButton, MONO_WIDTH, MONO_HEIGHT};
use std::time::{Instant, Duration};

fn main() {
    let mut lcd = Lcd::init_mono("Blink").unwrap();

    let blank_screen  = std::iter::repeat(0u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>();
    let filled_screen = std::iter::repeat(255u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>();

    lcd.set_mono_text(1, "     Press a button!").unwrap();

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        match lcd.is_button_pressed(LcdButton::MONO_BUTTON) {
            true => lcd.set_mono_background(&filled_screen[..]).unwrap(),
            false => lcd.set_mono_background(&blank_screen[..]).unwrap(),
        }
        lcd.update();
        std::thread::sleep(Duration::from_millis(100));
    }
}
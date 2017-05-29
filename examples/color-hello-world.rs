extern crate logitech_lcd;
use logitech_lcd::{Lcd, COLOR_WIDTH, COLOR_HEIGHT, COLOR_BYTES_PER_PIXEL};

fn main() {
    let blank_screen = std::iter::repeat(255u8).take(
        COLOR_WIDTH * COLOR_HEIGHT * COLOR_BYTES_PER_PIXEL).collect::<Vec<u8>>();

    let mut lcd = Lcd::init_color("Color image app").unwrap();
    lcd.set_color_background(&blank_screen[..]).unwrap();

    lcd.set_color_title("  Hello World!", 0, 0, 0).unwrap();

    lcd.set_color_text(0, "Red",     0xFF, 0x00, 0x00).unwrap();
    lcd.set_color_text(1, "Green",   0x00, 0xFF, 0x00).unwrap();
    lcd.set_color_text(2, "Blue",    0x00, 0x00, 0xFF).unwrap();
    lcd.set_color_text(3, "Yellow",  0xFF, 0xFF, 0x00).unwrap();
    lcd.set_color_text(4, "Cyan ",   0x00, 0xFF, 0xFF).unwrap();
    lcd.set_color_text(5, "Magenta", 0xFF, 0x00, 0xFF).unwrap();
    lcd.update();

    std::thread::sleep(std::time::Duration::from_millis(10000));
}

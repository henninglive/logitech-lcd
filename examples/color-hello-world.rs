extern crate logitech_lcd;
use logitech_lcd::{Driver, COLOR_WIDTH, COLOR_HEIGHT};

fn main() {
    let blank_screen = std::iter::repeat(255u8).take(
        COLOR_WIDTH * COLOR_HEIGHT * 4).collect::<Vec<u8>>();

    let mut driver = Driver::init_color("Color image app").unwrap();
    driver.set_color_background(&blank_screen[..]).unwrap();

    driver.set_color_title("  Hello World!", 0, 0, 0).unwrap();

    driver.set_color_text(0, "Red",     0xFF, 0x00, 0x00).unwrap();
    driver.set_color_text(1, "Green",   0x00, 0xFF, 0x00).unwrap();
    driver.set_color_text(2, "Blue",    0x00, 0x00, 0xFF).unwrap();
    driver.set_color_text(3, "Yellow",  0xFF, 0xFF, 0x00).unwrap();
    driver.set_color_text(4, "Cyan ",   0x00, 0xFF, 0xFF).unwrap();
    driver.set_color_text(5, "Magenta", 0xFF, 0x00, 0xFF).unwrap();
    driver.update();

    std::thread::sleep(std::time::Duration::from_millis(10000));
}

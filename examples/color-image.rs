extern crate image;
extern crate logitech_lcd;

use logitech_lcd::{Lcd, COLOR_WIDTH, COLOR_HEIGHT, COLOR_BYTES_PER_PIXEL};
use image::{ImageFormat, ImageRgba8};

fn load_image_into_buffer() -> Vec<u8> {
    let logo_data = include_bytes!("rust-logo-128x128.png");
    let logo_img = match image::load_from_memory_with_format(logo_data, ImageFormat::PNG).unwrap() {
        ImageRgba8(img) => img,
        _ => panic!("unexpected image format"),
    };

    let mut buf = std::iter::repeat(0u8)
        .take(COLOR_WIDTH * COLOR_HEIGHT * COLOR_BYTES_PER_PIXEL)
        .collect::<Vec<u8>>();

    let mx = (COLOR_WIDTH  - logo_img.width()  as usize) / 2;
    let my = (COLOR_HEIGHT - logo_img.height() as usize) / 2;

    for p in logo_img.enumerate_pixels() {
        let x = mx + p.0 as usize;
        let y = my + p.1 as usize;
        let i = y * COLOR_BYTES_PER_PIXEL * COLOR_WIDTH + x * COLOR_BYTES_PER_PIXEL;
        buf[i] = p.2.data[2];
        buf[i + 1] = p.2.data[1];
        buf[i + 2] = p.2.data[0];
        buf[i + 3] = p.2.data[3];
    }

    buf
}


fn main() {
    let buf = load_image_into_buffer();

    let mut lcd = Lcd::init_color("Color image app").unwrap();
    lcd.set_color_background(&buf[..]).unwrap();
    lcd.update();

    std::thread::sleep(std::time::Duration::from_millis(10000));
}
extern crate image;
extern crate logitech_lcd;

use logitech_lcd::{Driver, MONO_WIDTH, MONO_HEIGHT};
use image::{ImageFormat, ImageRgba8};

fn load_image_into_buffer() -> Vec<u8> {
    let logo_data = include_bytes!("rust-logo-32x32-blk.png");
    let logo_img = match image::load_from_memory_with_format(logo_data, ImageFormat::PNG).unwrap() {
        ImageRgba8(img) => img,
        _ => panic!("unexpected image format"),
    };

    let mut buf = std::iter::repeat(0u8)
        .take(MONO_WIDTH * MONO_HEIGHT)
        .collect::<Vec<u8>>();

    let my = (MONO_HEIGHT - logo_img.height() as usize) / 2;
    let mx = MONO_WIDTH - logo_img.width() as usize;

    for p in logo_img.enumerate_pixels() {
        let x = p.0 as usize;
        let y = my + p.1 as usize;
        buf[y * MONO_WIDTH + x] = p.2.data[3];
    }

    for p in logo_img.enumerate_pixels() {
        let x = mx + p.0 as usize;
        let y = my + p.1 as usize;
        buf[y * MONO_WIDTH + x] = p.2.data[3];
    }

    buf
}


fn main() {
    let buf = load_image_into_buffer();

    let mut driver = Driver::init_mono("Mono image app").unwrap();
    driver.set_mono_text(1, "      Rust is Awesome").unwrap();
    driver.set_mono_background(&buf[..]).unwrap();
    driver.update();

    std::thread::sleep(std::time::Duration::from_millis(10000));
}
//! Tests if Lcd is Send.

extern crate logitech_lcd;

use logitech_lcd::{Driver, MONO_WIDTH, MONO_HEIGHT};
use std::sync::Arc;
use std::thread;
use std::iter;

fn blink(driver: &mut Driver, blank: &[u8], filled: &[u8]) {
    for i in 0..10 {
        match i % 2 == 0 {
            true  => driver.set_mono_background(blank).unwrap(),
            false => driver.set_mono_background(filled).unwrap(),
        }
        driver.update();
        thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn main() {
    let mut driver = Driver::init_mono("Threaded").unwrap();
    let blank = Arc::new(iter::repeat(0u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>());
    let filled = Arc::new(iter::repeat(255u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>());

    let blank2 = blank.clone();
    let filled2 = filled.clone();

    let child = thread::spawn(move || {
        blink(&mut driver, &(*blank2)[..], &(*filled2)[..]);
        driver
    });

    driver = child.join().unwrap();
    blink(&mut driver, &(*blank)[..], &(*filled)[..]);
}
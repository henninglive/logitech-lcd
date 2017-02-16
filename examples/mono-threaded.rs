extern crate logi_lcd;

use logi_lcd::{MonoLcd, MONO_WIDTH, MONO_HEIGHT};
use std::sync::Arc;
use std::thread;
use std::iter;

fn blink(lcd: &mut MonoLcd, blank: &[u8], filled: &[u8]) {
    for i in 0..10 {
        match i % 2 == 0 {
            true  => lcd.set_background(blank).unwrap(),
            false => lcd.set_background(filled).unwrap(),
        }
        lcd.update();
        thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn main() {
    let mut lcd = MonoLcd::connect("Threaded").unwrap();
    let blank = Arc::new(iter::repeat(0u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>());
    let filled = Arc::new(iter::repeat(255u8).take(MONO_WIDTH * MONO_HEIGHT).collect::<Vec<u8>>());
    
    let blank2 = blank.clone();
    let filled2 = filled.clone();

    let child = thread::spawn(move || {
        blink(&mut lcd, &(*blank2)[..], &(*filled2)[..]);
        lcd
    });

    lcd = child.join().unwrap();
    blink(&mut lcd, &(*blank)[..], &(*filled)[..]);
}
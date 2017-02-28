extern crate logi_lcd;
use logi_lcd::{Lcd, LcdTypeMono};

fn main() {
    let mut lcd = Lcd::<LcdTypeMono>::connect_mono("Hello World").unwrap();
    lcd.set_mono_text(1, "        Hello World!").unwrap();
    lcd.update();

    std::thread::sleep(std::time::Duration::from_millis(5000));
}

extern crate logi_lcd;
use logi_lcd::MonoLcd;

fn main() {
    let mut lcd = MonoLcd::connect("Hello World").unwrap();
    lcd.set_text(1, "        Hello World!").unwrap();
    lcd.update();

    std::thread::sleep(std::time::Duration::from_millis(5000));
}

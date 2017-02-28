extern crate logi_lcd;

fn main() {
    let mut lcd = logi_lcd::init_mono("Hello World").unwrap();
    lcd.set_mono_text(1, "        Hello World!").unwrap();
    lcd.update();

    std::thread::sleep(std::time::Duration::from_millis(5000));
}

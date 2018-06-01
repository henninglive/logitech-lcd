extern crate logitech_lcd;

fn main() {
    let mut driver = logitech_lcd::Driver::init_mono("Hello World").unwrap();

    driver.set_mono_text(1, "        Hello World!").unwrap();
    driver.update();

    std::thread::sleep(std::time::Duration::from_millis(5000));
}

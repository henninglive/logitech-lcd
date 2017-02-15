# logi-lcd
FFI bindings for the Logitech Gaming LCD/Gamepanel SDK

## Overview
The Logitech LCD/GamePanel SDK introduces second screen capability that allows GamePanel-enabled Logitech gaming keyboards to display in-game info, system statistics, and more. The SDK enables integration of GamePanel functionality within your code.

## Supported Devices
- G19 - 320x240 Full RGBA **(Untested)**
- G510 - 160x43 Monochrome **(Tested)**
- G13 - 160x43 Monochrome **(Untested)**
- G15 v1 - 160x43 Monochrome **(Untested)**
- G15 v2 - 160x43 Monochrome **(Untested)**

## Building
### Requirements
- **LogitechLcd.dll** - Install [Logitech Gaming Software](http://support.logitech.com/en_us/software/lgs) or manually download the [LCD/Gamepanel SDK](http://gaming.logitech.com/en-us/developers))

### Linking
The build script will try to find the LogitechLcd.dll by searching the windows registry for the CLSID associated with LogitechLcd.dll. You can specify a custom location by [overriding the build script](http://doc.crates.io/build-script.html#overriding-build-scripts) in any acceptable Cargo [configuration location](http://doc.crates.io/config.html).

### Dependencies
Your output binary will have a dependency on LogitechLcd.dll. Logitech Gaming Software will not add the LCD/Gamepanel SDK to your PATH environment variable. Make sure you add the SDK to your PATH or copy the LogitechLcd.dll into same folder as your output binary.

## Example usage
```rust
extern crate logi_lcd;
use logi_lcd::MonoLcd;

fn main() {
    let mut lcd = MonoLcd::connect("Hello World").unwrap();
    lcd.set_text(0, "Hello World!").unwrap();
    lcd.update();

    std::thread::sleep(std::time::Duration::from_millis(5000));
}

```

## License
[MIT](./LICENSE)
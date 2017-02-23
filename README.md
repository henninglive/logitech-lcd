# logi-lcd
[![Build status](https://ci.appveyor.com/api/projects/status/yeblonuvclkd7n9e?svg=true)](https://ci.appveyor.com/project/henninglive/logi-lcd)

FFI bindings for the [Logitech Gaming LCD/Gamepanel SDK](http://gaming.logitech.com/en-us/developers)

## Overview
The Logitech LCD/GamePanel SDK introduces second screen capability that allows GamePanel-enabled Logitech gaming keyboards to display in-game info, system statistics, and more. The SDK enables integration of GamePanel functionality within your code.

## Supported Devices
- G19 - 320x240 Full RGBA **(Untested)**
- G510 - 160x43 Monochrome **(Working)**
- G13 - 160x43 Monochrome **(Untested)**
- G15 v1 - 160x43 Monochrome **(Untested)**
- G15 v2 - 160x43 Monochrome **(Untested)**
- LCD emulator - 160x43 Monochrome  **(Working)**
- LCD emulator - 320x240 Full RGBA  **(Working)**

### LCD Emulator
The Logitech Gaming Software comes with an LCD emulator. You can access it by going to your task bar tray `CTRL + SHIFT + RIGHT CLICK` on Logitech Gaming Software tray icon and press "LCD Emulator"

## Building
### Requirements
- **[Logitech Gaming Software](http://support.logitech.com/en_us/software/lgs)**

### Linking
The build script will try to locate your Logitech Gaming Software install and link to the appropriate binaries. You can override this behavior and manually pointing to your SDK install with `LOGITECH_LCD_LIB_DIR` environment variable.
```cmd
Set LOGITECH_LCD_LIB_DIR=C:\Logitech\LCDSDK
```
You can also download the standalone SDK from [here](http://gaming.logitech.com/en-us/developers) if you want to build this library without installing the Logitech Gaming Software. The standalone SDK comes with a `LogitechLcdEnginesWrapper.dll` witch is import wrapper around `LogitechLcd.dll` witch comes with the Logitech Gaming Software. To link against the wrapper, you need to change its name to `LogitechLcd.dll` and point to it with the `LOGITECH_LCD_LIB_DIR` environment variable.

Do not link against the `.lib` files provided with standalone SDK, they contain mangled symbols. The build script will automatically create `.lib` import librares for MSVC builds.

### Copy to DLL to output 
Tell the build script to copy the LogitechLcd.dll to the target directory by setting the `LOGITECH_LCD_COPY_OUT` environment variable. The Logitech Gaming Software does not add the SDK binaries to PATH. Coping the DLL to the target directory allows your output binary to find the DLL.
```cmd
Set LOGITECH_LCD_COPY_OUT=TRUE
```

## Example usage
### Mono LCD
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
### Code
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Art
The Rust and Cargo logos (bitmap and vector) are owned by Mozilla and distributed under the terms of the [Creative Commons Attribution license (CC-BY)](https://creativecommons.org/licenses/by/4.0/)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

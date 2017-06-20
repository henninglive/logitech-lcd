# logitech-lcd
[![Build status](https://ci.appveyor.com/api/projects/status/sf8ladr0v2pdqigd?svg=true)](https://ci.appveyor.com/project/henninglive/logitech-lcd)

FFI bindings for the [Logitech Gaming LCD/Gamepanel SDK][SDK]

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

## Requirements
- **[Logitech Gaming Software][LGS]**

### Dynamic Loading
This crate will try to locate and load `LogitechLcd.dll` at runtime. We start by looking up the `CLSID` in the Windows registry, if it’s found we load the library with call to [`LoadLibrary()`][LoadLibrary] with the full path. If it’s fails we call [`LoadLibrary()`][LoadLibrary] with just the DLL name. This will search your `PATH` for the library.

## Example usage
### Mono LCD
```rust
extern crate logitech_lcd;
use logitech_lcd::MonoLcd;

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

[SDK]: http://gaming.logitech.com/en-us/developers
[LGS]: http://support.logitech.com/en_us/software/lgs
[LoadLibrary]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms684175.aspx

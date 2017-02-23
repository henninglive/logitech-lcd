# logi-lcd
[![Build status](https://ci.appveyor.com/api/projects/status/yeblonuvclkd7n9e?svg=true)](https://ci.appveyor.com/project/henninglive/logi-lcd)

FFI bindings for the [Logitech Gaming LCD/Gamepanel SDK](http://gaming.logitech.com/en-us/developers)

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
[MIT](./LICENSE)

//! logitech-lcd provides binding for the [Logitech Gaming LCD/Gamepanel SDK](http://gaming.logitech.com/en-us/developers).
//!
//! ## Overview
//! The Logitech LCD/GamePanel SDK introduces second screen capability that allows GamePanel-enabled
//! Logitech gaming keyboards to display in-game info, system statistics, and more.
//! The SDK enables integration of GamePanel functionality within your code.
//!
//! ## Lcd Interface
//! The SDK interface is implemented by the [Driver struct](struct.Driver.html). Create a new
//! [Driver](struct.Driver.html) at start of program. Update the screen with the provided methods.
//! The [Driver](struct.Driver.html) will automatically disconnect when the [Driver](struct.Driver.html)
//! is dropped.
//!
//! ## Examples
//!
//! Monochrome:
//!
//! ```no_run
//! let mut driver = logitech_lcd::Driver::init_mono("My Glorious Monochrome App").unwrap();
//!
//! for i in 0..{
//!     driver.set_mono_text(0, &format!("update:{}", i)[..]).unwrap();
//!
//!     driver.update();
//!
//!     std::thread::sleep(std::time::Duration::from_millis(15));
//! }
//! ```
//! Color:
//!
//! ```no_run
//! let mut driver = logitech_lcd::Driver::init_color("My Glorious Color App").unwrap();
//!
//! for i in 0..{
//!     driver.set_color_text(0, &format!("update:{}", i)[..], i as u8,
//!         (i >> 8) as u8, (i >> 16) as u8).unwrap();
//!
//!     driver.update();
//!
//!     std::thread::sleep(std::time::Duration::from_millis(15));
//! }
//! ```
//! Monochrome and Color:
//!
//! ```no_run
//! let mut driver = logitech_lcd::Driver::init_either("My Glorious App").unwrap();
//!
//! for i in 0..{
//!     driver.set_mono_text(0,  &format!("update:{}", i)[..]).unwrap();
//!
//!     driver.set_color_text(0, &format!("update:{}", i)[..], i as u8,
//!         (i >> 8) as u8, (i >> 16) as u8).unwrap();
//!
//!     driver.update();
//!
//!     std::thread::sleep(std::time::Duration::from_millis(15));
//! }
//! ```
//!
//! ## Error Handling
//! The underling Logitech LCD/GamePanel SDK does unfortunately not return any info on error.
//! We therefore only able report what function failed, but not why. See [Error](enum.Error.html)
//!
//! ## Do’s and Don’ts
//! These are a few guidelines that may help you implement 'better' support in your application:
//!
//! - For color use a splash screen when the application starts up.
//! - For color have a nice background image to take full advantage of the RGBA LCD.
//! - Don’t just display information on the LCD that is already being displayed on main view of your
//! application. Instead display information he can only see when hitting tab or going to the menu.
//! - Use the LCD to unclutter the main view.
//! - Write support for both the color and monochrome LCDs, as both have an important user base.
//! - Text displayed on the LCD is fixed-width, so you can easily create multiple columns that
//! always align correctly.
//! - If you want to create custom screens, draw your own bitmaps and update the background LCD
//! bitmap up to 60 times/second.
//! - Use the buttons to create multiple pages or add functionality to the LCD.
//!
#![warn(missing_docs)]

extern crate logitech_lcd_sys as sys;

pub use sys::{
    LcdButton, MONO_WIDTH, MONO_HEIGHT, COLOR_WIDTH, COLOR_HEIGHT,
};

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::os::raw::c_int;

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Main LCD interface
///
/// Initialize at start of your program. Can Be initialized with color support,
/// monochrome support and both. Will automatically disconnect when the Lcd is dropped.
#[derive(Debug)]
pub struct Driver {
    type_flags: sys::LcdType,
    lib: sys::Library,
}

/// Runtime LCD error
///
/// The underling Logitech LCD/GamePanel SDK does unfortunately not return any info on error.
/// We therefore only able report what function failed, but not why.
#[derive(Debug)]
pub enum Error {
    /// A logitech LCD is not connected to the system.
    NotConnected,
    /// FFI call to LogiLcdInit() in LogitechLcd.dll has failed.
    Initialization,
    /// FFI call to LogiLcdMonoSetBackground() in LogitechLcd.dll has failed.
    MonoBackground,
    /// FFI call to LogiLcdMonoSetText() in LogitechLcd.dll has failed.
    MonoText,
    /// FFI call to LogiLcdColorSetBackground() in LogitechLcd.dll has failed.
    ColorBackground,
    /// FFI call to LogiLcdColorSetTitle() in LogitechLcd.dll has failed.
    ColorTitle,
    /// FFI call to LogiLcdColorSetText() in LogitechLcd.dll has failed.
    ColorText,
    /// Unexpected NULL character
    NullCharacter,
    /// Failed to load LogitechLcd.dll.
    LoadLibrary(std::io::Error),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotConnected    => "A logitech LCD is not connected to the system.",
            Error::Initialization  => "FFI call to LogiLcdInit() in LogitechLcd.dll has failed.",
            Error::MonoBackground  => "FFI call to LogiLcdMonoSetBackground() in LogitechLcd.dll has failed.",
            Error::MonoText        => "FFI call to LogiLcdMonoSetText() in LogitechLcd.dll has failed.",
            Error::ColorBackground => "FFI call to LogiLcdColorSetTitle() in LogitechLcd.dll has failed.",
            Error::ColorTitle      => "FFI call to LogiLcdColorSetTitle() in LogitechLcd.dll has failed.",
            Error::ColorText       => "FFI call to LogiLcdColorSetText() in LogitechLcd.dll has failed.",
            Error::NullCharacter   => "Unexpected NULL character.",
            Error::LoadLibrary(_)  => "Failed to load LogitechLcd.dll",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::LoadLibrary(ref e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        match self.cause() {
            Some(c) => write!(f, "LcdError: {}, Cause: {}", self.description(), c.description()),
            None => write!(f, "LcdError: {}", self.description()),
        }
    }
}

#[cfg(target_os = "windows")]
fn str_to_wchar_checked(s: &str) -> Result<Vec<u16>, Error> {
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;

    // Check for null character
    if s.chars().any(|c| c as u32 == 0) {
        return Err(Error::NullCharacter);
    }

    // Encode as widechar/utf-16 and terminate with \0\0
    Ok(OsStr::new(s).encode_wide().chain(Some(0)).collect::<Vec<u16>>())
}


#[cfg(not(target_os = "windows"))]
fn str_to_wchar_checked(_: &str) -> Result<Vec<u16>, Error> {
    unimplemented!();
}

impl Driver {
    fn init(app_name: &str, type_flags: sys::LcdType) -> Result<Driver, Error> {
        let lib = sys::Library::load().map_err(|e| Error::LoadLibrary(e))?;
        let ws = str_to_wchar_checked(app_name)?;

        assert_eq!(INITIALIZED.swap(true, Ordering::SeqCst), false);
        let ret = unsafe {
            match (lib.LogiLcdInit)(ws.as_ptr(), type_flags.bits()) {
                true => {
                    match (lib.LogiLcdIsConnected)(type_flags.bits()) {
                        true => Ok(Driver {
                            type_flags: type_flags,
                            lib: lib,
                        }),
                        false => Err(Error::NotConnected),
                    }
                },
                false => Err(Error::Initialization),
            }
        };

        if ret.is_err() {
            INITIALIZED.store(false, Ordering::SeqCst);
        }

        ret
    }

    /// Initialize and connect to a monochrome lcd device.
    ///
    /// Parameters:
    /// - app_name: The name of your applet.
    ///
    /// Panics:
    /// - If another Lcd instance is alive.
    ///
    pub fn init_mono(app_name: &str) -> Result<Driver, Error>  {
        Self::init(app_name, sys::LcdType::MONO)
    }

    /// Initialize and connect to a color lcd device.
    ///
    /// Parameters:
    /// - app_name: The name of your applet.
    ///
    /// Panics:
    /// - If another Lcd instance is alive.
    ///
    pub fn init_color(app_name: &str) -> Result<Driver, Error>  {
        Self::init(app_name, sys::LcdType::COLOR)
    }

    /// Initialize and connect to either a monochrome or color lcd device.
    ///
    /// Parameters:
    /// - app_name: The name of your applet.
    ///
    /// Panics:
    /// - If another Lcd instance is alive.
    ///
    pub fn init_either(app_name: &str) -> Result<Driver, Error> {
        Self::init(app_name, sys::LcdType::EITHER)
    }

    /// Checks if the device is connected.
    ///
    /// Return value:
    /// If a device supporting the lcd type specified is found, it returns `true`, otherwise `false`
    ///
    pub fn is_connected(&self) -> bool {
        unsafe {
            (self.lib.LogiLcdIsConnected)(self.type_flags.bits())
        }
    }

    /// Updates the lcd display.
    ///
    /// You have to call this function every frame of your main loop, to keep the lcd updated.
    ///
    pub fn update(&mut self) {
        unsafe {
            (self.lib.LogiLcdUpdate)();
        }
    }

    /// Checks if the buttons specified by the parameter are being pressed.
    ///
    /// If the buttons specified are being pressed it returns `true`, otherwise `false`.
    /// The button will be considered pressed only if your applet is the one currently in the foreground.
    ///
    pub fn is_button_pressed(&self, buttons: LcdButton) -> bool {
        unsafe {
            (self.lib.LogiLcdIsButtonPressed)(buttons.bits())
        }
    }

    /// Sets the specified image as background for the monochrome lcd device.
    ///
    /// Parameters:
    /// - mono_bitmap: The image data is organized as a rectangular area, 160 bytes wide and 43
    /// bytes high. Despite the display being monochrome, 8 bits per pixel are used
    /// here for simple manipulation of individual pixels. A pixel will turn on the
    /// if the value assigned to that byte is >= 128, it will remain off if the value is < 128.
    ///
    /// Panics:
    /// - If mono_bitmap's length is not 160x43 bytes.
    /// - If Lcd was initialized without mono support.
    ///
    pub fn set_mono_background(&mut self, mono_bitmap: &[u8]) -> Result<(), Error> {
        assert!(!(self.type_flags | sys::LcdType::MONO).is_empty());
        assert_eq!(mono_bitmap.len(), MONO_WIDTH * MONO_HEIGHT);

        unsafe {
            match (self.lib.LogiLcdMonoSetBackground)(mono_bitmap.as_ptr()) {
                true => Ok(()),
                false => Err(Error::MonoBackground),
            }
        }
    }

    /// Sets the specified text in the requested line on the monochrome lcd device.
    ///
    /// Parameters:
    /// - line_number: The line on the screen you want the text to appear. The monochrome lcd display
    ///   has 4 lines, so this parameter can be any number from 0 to 3.
    /// - **text**: Defines the text you want to display
    ///
    /// Panics:
    /// - If line_number larger than or equal to 4.
    /// - If Lcd was initialized without mono support.
    ///
    pub fn set_mono_text(&mut self, line_number: usize, text: &str) -> Result<(), Error> {
        assert!(!(self.type_flags | sys::LcdType::MONO).is_empty());

        let ws = str_to_wchar_checked(text)?;
        assert!(line_number < 4);

        unsafe {
            match (self.lib.LogiLcdMonoSetText)(line_number as c_int, ws.as_ptr()) {
                true => Ok(()),
                false => Err(Error::MonoText),
            }
        }
    }

    /// Sets the specified image as background for the color lcd device connected.
    ///
    /// Parameters:
    /// - color_bitmap: ARGB color bitmap, full RGB gamma, 8-bit per channel,
    /// 320 pixels wide and 240 pixels high, 32 bits per pixel(4 bytes).
    ///
    /// Panics:
    /// - If color_bitmap's length is not 320x240x4 bytes.
    /// - If Lcd was initialized without color support.
    ///
    pub fn set_color_background(&mut self, color_bitmap: &[u8]) -> Result<(), Error> {
        assert!(!(self.type_flags | sys::LcdType::COLOR).is_empty());
        assert_eq!(color_bitmap.len(), COLOR_WIDTH * COLOR_HEIGHT * 4);

        unsafe {
            match (self.lib.LogiLcdColorSetBackground)(color_bitmap.as_ptr()) {
                true => Ok(()),
                false => Err(Error::ColorBackground),
            }
        }
    }

    /// Sets the specified text in the first line on the color lcd device connected.
    /// The font size that will be displayed is bigger than the one used in the other lines,
    /// so you can use this function to set the title of your applet/page.
    ///
    /// Parameters:
    /// - text: Defines the text you want to display as title.
    /// - red, green, blue: The LCD can display a full RGB color, you can define the color
    /// of your title using these parameters.
    ///
    /// Panics:
    /// - If Lcd was initialized without color support.
    ///
    pub fn set_color_title(&mut self, text: &str, red: u8, green: u8, blue: u8)
        -> Result<(), Error>
    {
        assert!(!(self.type_flags | sys::LcdType::COLOR).is_empty());
        let ws = str_to_wchar_checked(text)?;

        unsafe {
            match (self.lib.LogiLcdColorSetTitle)(ws.as_ptr(), red as c_int,
                green as c_int, blue as c_int)
            {
                true  => Ok(()),
                false => Err(Error::ColorTitle),
            }
        }
    }

    /// Sets the specified text in the requested line on the color lcd device connected.
    ///
    /// Parameters:
    /// - line_number: The line on the screen you want the text to appear. The color lcd display
    /// has 8 lines, or standard text, so this parameter can be any number from 0 to 7
    /// - text: Defines the text you want to display as title.
    /// - red, green, blue: The LCD can display a full RGB color, you can define the color
    /// of your title using these parameters.
    ///
    /// Panics:
    /// - If line_number larger than or equal to 8.
    /// - If Lcd was initialized without color support.
    ///
    pub fn set_color_text(&mut self, line_number: usize, text: &str,
        red: u8, green: u8, blue: u8) -> Result<(), Error>
    {
        assert!(!(self.type_flags | sys::LcdType::COLOR).is_empty());

        let ws = str_to_wchar_checked(text)?;
        assert!(line_number < 8);

        unsafe {
            match (self.lib.LogiLcdColorSetText)(line_number as c_int,
                ws.as_ptr(), red as c_int, green as c_int, blue as c_int)
            {
                true => Ok(()),
                false => Err(Error::ColorText),
            }
        }
    }
}

impl Drop for Driver {
    /// Kills the applet and frees memory used by the SDK
    fn drop(&mut self) {
        unsafe {
            (self.lib.LogiLcdShutdown)();
        }
        INITIALIZED.store(false, Ordering::SeqCst);
    }
}

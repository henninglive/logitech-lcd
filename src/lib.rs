#![warn(missing_docs)]

//! # logi-lcd
//! logi-lcd provides binding for the [Logitech Gaming LCD/Gamepanel SDK]
//! (http://gaming.logitech.com/en-us/developers).
//!
//! ## Overview
//! The Logitech LCD/GamePanel SDK introduces second screen capability that allows GamePanel-enabled
//! Logitech gaming keyboards to display in-game info, system statistics, and more.
//! The SDK enables integration of GamePanel functionality within your code.
//!
//! ## Lcd Interface
//! The SDK interface is implemented by the [Lcd](struct.Lcd.html) struct. Create a new
//! [Lcd](struct.Lcd.html) at start of program. Update it with the provided methods.
//! The [Lcd](struct.Lcd.html) will automatically disconnect when the [Lcd](struct.Lcd.html)
//! goes out of scope.
//!
//! ## Example
//! ```
//! let mut lcd = logi_lcd::Lcd::init_mono("My Glorious App").unwrap();
//!
//! lcd.set_mono_text(0, "Hello World!").unwrap();
//!
//! lcd.update();
//! ```
//!
//! ## Error Handling
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

extern crate logi_lcd_sys as sys;

pub use sys::{
    LcdButton, LcdType, BitFlags,
    MONO_WIDTH, MONO_HEIGHT, MONO_BYTES_PER_PIXEL,
    COLOR_WIDTH, COLOR_HEIGHT, COLOR_BYTES_PER_PIXEL
};

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::os::raw::c_int;

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

pub struct Lcd {
    type_flags: BitFlags<LcdType>,
}

#[derive(Debug)]
pub enum LcdError {
    NotConnected,
    Initialization,
    MonoBackground,
    MonoText,
    ColorBackground,
    ColorTitle,
    ColorText,
    NullCharacter,
}

impl Error for LcdError {
    fn description(&self) -> &str {
        match *self {
            LcdError::NotConnected    => "LCD is not connected",
            LcdError::Initialization  => "A FFI call to LogiLcdInit() has failed",
            LcdError::MonoBackground  => "A FFI call to LogiLcdMonoSetBackground() has failed",
            LcdError::MonoText        => "A FFI call to LogiLcdMonoSetText() has failed",
            LcdError::ColorBackground => "A FFI call to LogiLcdColorSetBackground() has failed",
            LcdError::ColorTitle      => "A FFI call to LogiLcdColorSetTitle() has failed",
            LcdError::ColorText       => "A FFI call to LogiLcdColorSetText() has failed",
            LcdError::NullCharacter   => "Unexpected NULL character",
        }
    }
}

impl Display for LcdError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "LcdError: {}", self.description())
    }
}

fn str_to_wchar(s: &str) -> Result<Vec<u16>, LcdError> {
    let mut v = s.encode_utf16().collect::<Vec<u16>>();

    if v.iter().any(|&val| val == 0) {
        return Err(LcdError::NullCharacter);
    }

    v.push(0);

    Ok(v)
}

impl Lcd {
    fn init(app_name: &str, type_flags: BitFlags<LcdType>) -> Result<Lcd, LcdError> {
        assert_eq!(INITIALIZED.swap(true, Ordering::SeqCst), false);
        let ws = str_to_wchar(app_name)?;

        let ret = unsafe {
            match sys::LogiLcdInit(ws.as_ptr(), type_flags.bits()) {
                true => {
                    match sys::LogiLcdIsConnected(type_flags.bits()) {
                        true => Ok(Lcd{type_flags: type_flags}),
                        false => Err(LcdError::NotConnected),
                    }
                },
                false => Err(LcdError::Initialization),
            }
        };

        if ret.is_err() {
            INITIALIZED.store(false, Ordering::SeqCst);
        }

        ret
    }

    /// Initialize and connect to a monochrome lcd device.
    ///
    /// ### Parameters:
    /// - app_name: The name of your applet.
    ///
    /// ### Panics
    /// Will panic if another lcd instance exits.
    ///
    pub fn init_mono(app_name: &str) -> Result<Lcd, LcdError>  {
        Self::init(app_name, LcdType::MONO.into())
    }

    /// Initialize and connect to a color lcd device.
    ///
    /// ### Parameters:
    /// - app_name: The name of your applet.
    ///
    /// ### Panics
    /// Will panic if another lcd instance exits.
    ///
    pub fn init_color(app_name: &str) -> Result<Lcd, LcdError>  {
        Self::init(app_name, LcdType::COLOR.into())
    }

    /// Initialize and connect to either a monochrome or color lcd device.
    ///
    /// ### Parameters:
    /// - app_name: The name of your applet.
    ///
    /// ### Panics
    /// Will panic if another lcd instance exits.
    ///
    pub fn init_either(app_name: &str) -> Result<Lcd, LcdError> {
        Self::init(app_name, LcdType::either())
    }

    /// Checks if the device is connected.
    ///
    /// ### Return value:
    /// If a device supporting the lcd type specified is found, it returns `true`. Otherwise `false`
    ///
    pub fn is_connected(&self) -> bool {
        unsafe {
            sys::LogiLcdIsConnected(self.type_flags.bits())
        }
    }

    /// Updates the lcd display.
    ///
    /// ### Notes:
    /// You have to call this function every frame of your main loop, to keep the lcd updated.
    ///
    pub fn update(&mut self) {
        unsafe {
            sys::LogiLcdUpdate();
        }
    }

    /// Checks if the buttons specified by the parameter are being pressed.
    ///
    /// ### Return value:
    /// If the button specified is being pressed it returns `true`. Otherwise `false`
    ///
    /// ### Notes:
    /// The button will be considered pressed only if your applet is the one currently in the foreground.
    ///
    pub fn is_mono_buttons_pressed(&self, buttons: BitFlags<LcdButton>) -> bool {
        assert!(!(self.type_flags | LcdType::MONO).is_empty());

        unsafe {
            sys::LogiLcdIsButtonPressed((buttons & LcdButton::mono()).bits())
        }
    }

    /// Sets the specified image as background for the monochrome lcd device.
    ///
    /// ### Parameters:
    /// - bytemap: The image data is organized as a rectangular area, 160 bytes wide and 43
    /// bytes high. Despite the display being monochrome, 8 bits per pixel are used
    /// here for simple manipulation of individual pixels. The SDK will turn on the
    /// pixel on the screen if the value assigned to that byte is >= 128, it will
    /// remain off if the value is < 128.
    ///
    /// ### Panics
    /// Will panic if bytemaps size is not 160x43bytes long.
    ///
    pub fn set_mono_background(&mut self, bytemap: &[u8]) -> Result<(), LcdError> {
        assert!(!(self.type_flags | LcdType::MONO).is_empty());
        assert_eq!(bytemap.len(), MONO_WIDTH * MONO_HEIGHT);

        unsafe {
            match sys::LogiLcdMonoSetBackground(bytemap.as_ptr()) {
                true => Ok(()),
                false => Err(LcdError::MonoBackground),
            }
        }
    }

    /// Sets the specified text in the requested line on the monochrome lcd device.
    ///
    /// ### Parameters:
    /// - line_number: The line on the screen you want the text to appear. The monochrome lcd display
    ///   has 4 lines, so this parameter can be any number from 0 to 3.
    /// - text: Defines the text you want to display
    ///
    /// ### Panics
    /// Will panic if line_number larger than or equal to 4.
    ///
    pub fn set_mono_text(&mut self, line_number: usize, text: &str) -> Result<(), LcdError> {
        assert!(!(self.type_flags | LcdType::MONO).is_empty());

        let ws = str_to_wchar(text)?;
        assert!(line_number < 4);

        unsafe {
            match sys::LogiLcdMonoSetText(line_number as c_int, ws.as_ptr()) {
                true => Ok(()),
                false => Err(LcdError::MonoText),
            }
        }
    }

    /// Checks if the buttons specified by the parameter are being pressed.
    ///
    /// ### Return value:
    /// If the button specified is being pressed it returns `true`. Otherwise `false`
    ///
    /// ### Notes:
    /// The button will be considered pressed only if your applet is the one currently in the foreground.
    ///
    pub fn is_color_buttons_pressed(&self, buttons: BitFlags<LcdButton>) -> bool {
        assert!(!(self.type_flags | LcdType::COLOR).is_empty());

        unsafe {
            sys::LogiLcdIsButtonPressed((buttons & LcdButton::color()).bits())
        }
    }

    pub fn set_color_background(&mut self, bitmap: &[u8]) -> Result<(), LcdError> {
        assert!(!(self.type_flags | LcdType::COLOR).is_empty());
        assert_eq!(bitmap.len(), COLOR_WIDTH * COLOR_HEIGHT * COLOR_BYTES_PER_PIXEL);

        unsafe {
            match sys::LogiLcdColorSetBackground(bitmap.as_ptr()) {
                true => Ok(()),
                false => Err(LcdError::ColorBackground),
            }
        }
    }

    pub fn set_color_title(&mut self, text: &str, red: u8, green: u8, blue: u8)
        -> Result<(), LcdError>
    {
        assert!(!(self.type_flags | LcdType::COLOR).is_empty());
        let ws = str_to_wchar(text)?;

        unsafe {
            match sys::LogiLcdColorSetTitle(ws.as_ptr(), red as c_int,
                green as c_int, blue as c_int)
            {
                true  => Ok(()),
                false => Err(LcdError::ColorTitle),
            }
        }
    }

    pub fn set_color_text(&mut self, line_number: usize, text: &str,
        red: u8, green: u8, blue: u8) -> Result<(), LcdError>
    {
        assert!(!(self.type_flags | LcdType::COLOR).is_empty());

        let ws = str_to_wchar(text)?;
        assert!(line_number < 8);

        unsafe {
            match sys::LogiLcdColorSetText(line_number as c_int,
                ws.as_ptr(), red as c_int, green as c_int, blue as c_int)
            {
                true => Ok(()),
                false => Err(LcdError::ColorText),
            }
        }
    }
}

impl Drop for Lcd {
    /// Kills the applet and frees memory used by the SDK
    fn drop(&mut self) {
        unsafe {
            sys::LogiLcdShutdown();
        }
        INITIALIZED.store(false, Ordering::SeqCst);
    }
}

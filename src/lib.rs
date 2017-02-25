extern crate logi_lcd_sys as sys;
extern crate enumflags;
#[macro_use] extern crate enumflags_derive;

use sys::*;
use enumflags::{BitFlags, InnerBitFlags};

use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub const MONO_WIDTH: usize   = sys::LOGI_LCD_MONO_WIDTH;
pub const MONO_HEIGHT: usize  = sys::LOGI_LCD_MONO_HEIGHT;
pub const MONO_BYTES_PER_PIXEL: usize = sys::LOGI_LCD_MONO_PXL_BSIZE;

pub const COLOR_WIDTH: usize  = sys::LOGI_LCD_COLOR_WIDTH;
pub const COLOR_HEIGHT: usize = sys::LOGI_LCD_COLOR_HEIGHT;
pub const COLOR_BYTES_PER_PIXEL: usize = sys::LOGI_LCD_COLOR_PXL_BSIZE;


static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

pub struct MonoLcd;
pub struct ColorLcd;

#[repr(u32)]
#[derive(EnumFlags, Copy, Clone, Debug)]
enum LcdType {
    Mono  = 0x00000001,
    Color = 0x00000002,
}

#[repr(u32)]
#[derive(EnumFlags, Copy, Clone, Debug)]
pub enum MonoButton {
    Button0      = 0x00000001,
    Button1      = 0x00000002,
    Button2      = 0x00000004,
    Button3      = 0x00000008,
}

#[repr(u32)]
#[derive(EnumFlags, Copy, Clone, Debug)]
pub enum ColorButton {
    ButtonLeft   = 0x00000001,
    ButtonRight  = 0x00000002,
    ButtonOk     = 0x00000004,
    ButtonCancel = 0x00000008,
    ButtonUp     = 0x00000010,
    ButtonDown   = 0x00000020,
    BttonMenu    = 0x00000040,
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


impl MonoLcd {
    /// Initialize and connect to a monochrome lcd device.
    ///
    /// ### Parameters:
    /// - app_name: The name of your applet.
    ///
    /// ### Panics
    /// Will panic if another lcd instance exits.
    ///
    pub fn connect(app_name: &str) -> Result<MonoLcd, LcdError> {
        assert_eq!(INITIALIZED.swap(true, Ordering::SeqCst), false);

        let ws = str_to_wchar(app_name)?;

        let ret = unsafe {
            match LogiLcdInit(ws.as_ptr(), LcdType::Mono.into()) {
                true => {
                    match LogiLcdIsConnected(LcdType::Color.into()) {
                        true => Ok(MonoLcd),
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

    /// Checks if the device is connected.
    ///
    /// ### Return value:
    /// If a device supporting the lcd type specified is found, it returns `true`. Otherwise `false`
    ///
    pub fn is_connected() -> bool {
        unsafe {
            LogiLcdIsConnected(LcdType::Mono.into())
        }
    }

    /// Checks if the button specified by the parameter is being pressed.
    ///
    /// ### Return value:
    /// If the button specified is being pressed it returns `true`. Otherwise `false`
    ///
    /// ### Notes:
    /// The button will be considered pressed only if your applet is the one currently in the foreground.
    ///
    pub fn is_button_pressed(&self, button: MonoButton) -> bool {
        unsafe {
            LogiLcdIsButtonPressed(button.into())
        }
    }

    /// Updates the lcd display.
    ///
    /// ### Notes:
    /// You have to call this function every frame of your main loop, to keep the lcd updated.
    ///
    pub fn update(&mut self) {
        unsafe {
            LogiLcdUpdate();
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
    pub fn set_background(&mut self, bytemap: &[u8]) -> Result<(), LcdError> {
        assert_eq!(bytemap.len(), MONO_WIDTH * MONO_HEIGHT);
        unsafe {
            match LogiLcdMonoSetBackground(bytemap.as_ptr()) {
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
    pub fn set_text(&mut self, line_number: usize, text: &str) -> Result<(), LcdError> {
        let ws = str_to_wchar(text)?;
        assert!(line_number < 4);
        unsafe {
            match LogiLcdMonoSetText(line_number as c_int, ws.as_ptr()) {
                true => Ok(()),
                false => Err(LcdError::MonoText),
            }
        }
    }
}

impl ColorLcd {
    /// Initialize and connect to a color lcd device.
    ///
    /// ### Parameters:
    /// - app_name: The name of your applet.
    ///
    /// ### Panics
    /// Will panic if another lcd instance exits.
    ///
    pub fn connect(app_name: &str) -> Result<ColorLcd, LcdError> {
        assert_eq!(INITIALIZED.swap(true, Ordering::SeqCst), false);

        let ws = str_to_wchar(app_name)?;

        let ret = unsafe {
            match LogiLcdInit(ws.as_ptr(), LcdType::Color.into()) {
                true => {
                    match LogiLcdIsConnected(LcdType::Color.into()) {
                        true => Ok(ColorLcd),
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

    /// Checks if the device is connected.
    ///
    /// ### Return value:
    /// If a device supporting the lcd type specified is found, it returns `true`. Otherwise `false`
    ///
    pub fn is_connected() -> bool {
        unsafe {
            LogiLcdIsConnected(LcdType::Color.into())
        }
    }

    /// Checks if the button specified by the parameter is being pressed.
    ///
    /// ### Return value:
    /// If the button specified is being pressed it returns `true`. Otherwise `false`
    ///
    /// ### Notes:
    /// The button will be considered pressed only if your applet is the one currently in the foreground.
    ///
    pub fn is_button_pressed(&self, button: ColorButton) -> bool {
        unsafe {
            let b: u32 = button.into();
            LogiLcdIsButtonPressed(b << 8)
        }
    }

    /// Updates the lcd display.
    ///
    /// ### Notes:
    /// You have to call this function every frame of your main loop, to keep the lcd updated.
    ///
    pub fn update(&mut self) {
        unsafe {
            LogiLcdUpdate();
        }
    }

    pub fn set_background(&mut self, bitmap: &[u8]) -> Result<(), LcdError> {
        assert_eq!(bitmap.len(), COLOR_WIDTH * COLOR_HEIGHT * COLOR_BYTES_PER_PIXEL);
        unsafe {
            match LogiLcdColorSetBackground(bitmap.as_ptr()) {
                true => Ok(()),
                false => Err(LcdError::ColorBackground),
            }
        }
    }

    pub fn set_title(&mut self, text: &str, red: u8, green: u8, blue: u8)
        -> Result<(), LcdError>
    {
        let ws = str_to_wchar(text)?;

        unsafe {
            match LogiLcdColorSetTitle(ws.as_ptr(), red as c_int,
                green as c_int, blue as c_int)
            {
                true  => Ok(()),
                false => Err(LcdError::ColorTitle),
            }
        }
    }

    pub fn set_text(&mut self, line_number: usize, text: &str,
        red: u8, green: u8, blue: u8) -> Result<(), LcdError>
    {
        let ws = str_to_wchar(text)?;
        assert!(line_number < 4);
        unsafe {
            match LogiLcdColorSetText(line_number as c_int,
                ws.as_ptr(), red as c_int, green as c_int, blue as c_int)
            {
                true => Ok(()),
                false => Err(LcdError::ColorText),
            }
        }
    }
}

impl Drop for MonoLcd {
    /// Kills the applet and frees memory used by the SDK
    fn drop(&mut self) {
        unsafe {
            LogiLcdShutdown();
        }
        INITIALIZED.store(false, Ordering::SeqCst);
    }
}

impl Drop for ColorLcd {
    /// Kills the applet and frees memory used by the SDK
    fn drop(&mut self) {
        unsafe {
            LogiLcdShutdown();
        }
        INITIALIZED.store(false, Ordering::SeqCst);
    }
}

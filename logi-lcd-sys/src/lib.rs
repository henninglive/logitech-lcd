#![allow(non_camel_case_types)]

#[macro_use]
extern crate enumflags_derive;
extern crate enumflags;

pub use enumflags::BitFlags;
use std::os::raw::{c_int, c_uint};

pub const MONO_WIDTH:  usize = 160;
pub const MONO_HEIGHT: usize = 43;
pub const MONO_BYTES_PER_PIXEL: usize = 1;
pub const COLOR_WIDTH:  usize = 320;
pub const COLOR_HEIGHT: usize = 240;
pub const COLOR_BYTES_PER_PIXEL: usize = 4;

#[derive(EnumFlags, Copy, Clone, Debug)]
#[repr(u32)]
pub enum LcdType {
    MONO  = 0x00000001,
    COLOR = 0x00000002,
}

#[derive(EnumFlags, Copy, Clone, Debug)]
#[repr(u32)]
pub enum LcdButton {
    MONO_BUTTON_0 = 0x00000001,
    MONO_BUTTON_1 = 0x00000002,
    MONO_BUTTON_2 = 0x00000004,
    MONO_BUTTON_3 = 0x00000008,
    COLOR_BUTTON_LEFT   = 0x00000100,
    COLOR_BUTTON_RIGHT  = 0x00000200,
    COLOR_BUTTON_OK     = 0x00000400,
    COLOR_BUTTON_CANCEL = 0x00000800,
    COLOR_BUTTON_UP     = 0x00001000,
    COLOR_BUTTON_DOWN   = 0x00002000,
    COLOR_BUTTON_MENU   = 0x00004000,
}

impl LcdType {
    pub fn either() -> BitFlags<LcdType> {
        LcdType::MONO | LcdType::COLOR
    }
}

impl LcdButton {
    pub fn mono() -> BitFlags<LcdButton> {
        LcdButton::MONO_BUTTON_0 |
        LcdButton::MONO_BUTTON_1 |
        LcdButton::MONO_BUTTON_2 |
        LcdButton::MONO_BUTTON_3
    }

    pub fn color() -> BitFlags<LcdButton> {
        LcdButton::COLOR_BUTTON_LEFT |
        LcdButton::COLOR_BUTTON_RIGHT |
        LcdButton::COLOR_BUTTON_OK |
        LcdButton::COLOR_BUTTON_CANCEL |
        LcdButton::COLOR_BUTTON_UP |
        LcdButton::COLOR_BUTTON_DOWN |
        LcdButton::COLOR_BUTTON_MENU
    }
}

#[link(name="LogitechLcd")]
extern "C" {
    pub fn LogiLcdInit(friendlyName: *const u16, lcdType: c_uint) -> bool;
    pub fn LogiLcdIsConnected(lcdType: c_uint) -> bool;
    pub fn LogiLcdIsButtonPressed(button: c_uint) -> bool;
    pub fn LogiLcdUpdate();
    pub fn LogiLcdShutdown();

    // Monochrome LCD functions
    pub fn LogiLcdMonoSetBackground(monoBitmap: *const u8) -> bool;
    pub fn LogiLcdMonoSetText(lineNumber: c_int, text: *const u16) -> bool;

    // Color LCD functions
    pub fn LogiLcdColorSetBackground(colorBitmap: *const u8) -> bool;
    pub fn LogiLcdColorSetTitle(text: *const u16, red: c_int, green: c_int,
        blue: c_int) -> bool;
    pub fn LogiLcdColorSetText(lineNumber: c_int, text: *const u16, red: c_int,
        green: c_int, blue: c_int) -> bool;

    //UDK functions, use this only if working with UDK
    pub fn LogiLcdColorSetBackgroundUDK(partialBitmap: *const u8, arraySize: c_int) -> c_int;
    pub fn LogiLcdColorResetBackgroundUDK() -> c_int;
    pub fn LogiLcdMonoSetBackgroundUDK(partialBitmap: *const u8, arraySize: c_int) -> c_int;
    pub fn LogiLcdMonoResetBackgroundUDK() -> c_int;
}

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate winapi;
use self::winapi::{BYTE, wchar_t, c_int};

pub const MONO_WIDTH: usize = 160;
pub const MONO_HEIGHT: usize = 43;
pub const COLOR_WIDTH: usize = 320;
pub const COLOR_HEIGHT: usize = 240;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bool {
    FALSE = 0, 
    TRUE = 1
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum LcdType {
    MONO  = 0x1,
    COLOR = 0x2,
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum LcdButton {
    MONO_BUTTON_0 = 0x1,
    MONO_BUTTON_1 = 0x2,
    MONO_BUTTON_2 = 0x4,
    MONO_BUTTON_3 = 0x8,
    COLOR_BUTTON_LEFT   = 0x100,
    COLOR_BUTTON_RIGHT  = 0x200,
    COLOR_BUTTON_OK     = 0x400,
    COLOR_BUTTON_CANCEL = 0x800,
    COLOR_BUTTON_UP     = 0x1000,
    COLOR_BUTTON_DOWN   = 0x2000,
    COLOR_BUTTON_MENU   = 0x4000,
}

#[link(name="LogitechLcd")]
extern "C" {
    pub fn LogiLcdInit(friendlyName: *const wchar_t, lcdType: LcdType) -> Bool;
    pub fn LogiLcdIsConnected(lcdType: LcdType) -> Bool;
    pub fn LogiLcdIsButtonPressed(button: LcdButton) -> Bool;
    pub fn LogiLcdUpdate();
    pub fn LogiLcdShutdown();

    // Monochrome LCD functions
    pub fn LogiLcdMonoSetBackground(monoBitmap: *const BYTE) -> Bool;
    pub fn LogiLcdMonoSetText(lineNumber: c_int, text: *const wchar_t) -> Bool;

    // Color LCD functions
    pub fn LogiLcdColorSetBackground(colorBitmap: *const BYTE) -> Bool;
    pub fn LogiLcdColorSetTitle(text: *const wchar_t, red: c_int, green: c_int, 
        blue: c_int) -> Bool;
    pub fn LogiLcdColorSetText(lineNumber: c_int, text: *const wchar_t, red: c_int, 
        green: c_int, blue: c_int) -> Bool;

    //UDK functions, use this only if working with UDK
    pub fn LogiLcdColorSetBackgroundUDK(partialBitmap: *const BYTE, arraySize: c_int) -> c_int;
    pub fn LogiLcdColorResetBackgroundUDK() -> c_int;
    pub fn LogiLcdMonoSetBackgroundUDK(partialBitmap: *const BYTE, arraySize: c_int) -> c_int;
    pub fn LogiLcdMonoResetBackgroundUDK() -> c_int;
}

impl From<Bool> for bool {
    fn from(b: Bool) -> bool {
        match b {
            Bool::FALSE => false,
            Bool::TRUE => true,
        }
    }
}

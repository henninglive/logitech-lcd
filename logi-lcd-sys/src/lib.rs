#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::os::raw::{c_int, c_uint};

pub const LOGI_LCD_MONO_WIDTH:     usize = 160;
pub const LOGI_LCD_MONO_HEIGHT:    usize = 43;
pub const LOGI_LCD_MONO_PXL_BSIZE: usize = 1;
pub const LOGI_LCD_COLOR_WIDTH:    usize = 320;
pub const LOGI_LCD_COLOR_HEIGHT:   usize = 240;
pub const LOGI_LCD_COLOR_PXL_BSIZE: usize = 4;

pub const LOGI_LCD_TYPE_MONO:  c_uint = 0x00000001;
pub const LOGI_LCD_TYPE_COLOR: c_uint = 0x00000002;

pub const LOGI_LCD_MONO_BUTTON_0:       c_uint = 0x00000001;
pub const LOGI_LCD_MONO_BUTTON_1:       c_uint = 0x00000002;
pub const LOGI_LCD_MONO_BUTTON_2:       c_uint = 0x00000004;
pub const LOGI_LCD_MONO_BUTTON_3:       c_uint = 0x00000008;


pub const LOGI_LCD_COLOR_BUTTON_LEFT:   c_uint = 0x00000100;
pub const LOGI_LCD_COLOR_BUTTON_RIGHT:  c_uint = 0x00000200;
pub const LOGI_LCD_COLOR_BUTTON_OK:     c_uint = 0x00000400;
pub const LOGI_LCD_COLOR_BUTTON_CANCEL: c_uint = 0x00000800;
pub const LOGI_LCD_COLOR_BUTTON_UP:     c_uint = 0x00001000;
pub const LOGI_LCD_COLOR_BUTTON_DOWN:   c_uint = 0x00002000;
pub const LOGI_LCD_COLOR_BUTTON_MENU:   c_uint = 0x00004000;

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

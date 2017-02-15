extern crate logi_lcd_sys as sys;
extern crate widestring;
extern crate winapi;

use widestring::WideCString;
use self::winapi::{c_int};
use sys::*;

pub const MONO_WIDTH: usize   = sys::MONO_WIDTH;
pub const MONO_HEIGHT: usize  = sys::MONO_HEIGHT;
pub const COLOR_WIDTH: usize  = sys::COLOR_WIDTH;
pub const COLOR_HEIGHT: usize = sys::COLOR_HEIGHT;

pub struct MonoLcd;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ColorButton {
    ButtonLeft,
    ButtonRight,
    ButtonOk,
    ButtonCancel,
    ButtonUp,
    ButtonDown,
    BttonMenu,
}

#[derive(Debug)]
pub enum MonoButton {
    Button0,
    Button1,
    Button2,
    Button3,
}

impl ColorButton {
    #[allow(dead_code)]
    fn lcd_button(self) -> LcdButton {
        match self {
            ColorButton::ButtonLeft   => LcdButton::COLOR_BUTTON_LEFT,
            ColorButton::ButtonRight  => LcdButton::COLOR_BUTTON_RIGHT,
            ColorButton::ButtonOk     => LcdButton::COLOR_BUTTON_OK,
            ColorButton::ButtonCancel => LcdButton::COLOR_BUTTON_CANCEL,
            ColorButton::ButtonUp     => LcdButton::COLOR_BUTTON_UP,
            ColorButton::ButtonDown   => LcdButton::COLOR_BUTTON_DOWN,
            ColorButton::BttonMenu    => LcdButton::COLOR_BUTTON_MENU,
        }
    }
}

impl MonoButton {
    fn lcd_button(self) -> LcdButton {
        match self {
            MonoButton::Button0 => LcdButton::MONO_BUTTON_0,
            MonoButton::Button1 => LcdButton::MONO_BUTTON_1,
            MonoButton::Button2 => LcdButton::MONO_BUTTON_2,
            MonoButton::Button3 => LcdButton::MONO_BUTTON_3,
        }
    }
}

impl MonoLcd {
    pub fn connect(app_name: &str) -> Result<MonoLcd, ()> {
        let ws = WideCString::from_str(app_name).unwrap();
        unsafe {
            match LogiLcdInit(ws.as_ptr(), LcdType::MONO) {
                Bool::TRUE => {
                    match LogiLcdIsConnected(LcdType::MONO) {
                        Bool::TRUE => Ok(MonoLcd),
                        Bool::FALSE => Err(()),
                    }
                },
                Bool::FALSE => Err(()),
            }
        }
    }

    pub fn is_button_pressed(&self, button: MonoButton) -> bool {
        unsafe {
            LogiLcdIsButtonPressed(button.lcd_button()).into()
        }
    }

    pub fn update(&mut self) {
        unsafe {
            LogiLcdUpdate();
        }
    }

    pub fn set_background(&mut self, bytemap: &[u8]) -> Result<(), ()> {
        assert_eq!(bytemap.len(), MONO_WIDTH * MONO_HEIGHT);
        unsafe {
            match LogiLcdMonoSetBackground(bytemap.as_ptr()) {
                Bool::TRUE  => Ok(()),
                Bool::FALSE => Err(()),
            }
        }
    }

    pub fn set_text(&mut self, line_number: usize, text: &str) -> Result<(), ()> {
        let ws = WideCString::from_str(text).unwrap();
        assert!(line_number < 4);
        unsafe {
            match LogiLcdMonoSetText(line_number as c_int, ws.as_ptr()) {
                Bool::TRUE  => Ok(()),
                Bool::FALSE => Err(()),
            }
        }
    }
}

impl Drop for MonoLcd {
    fn drop(&mut self) {
        unsafe {
            LogiLcdShutdown();
        }
    }
}

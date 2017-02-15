extern crate logi_lcd_sys as sys;
extern crate widestring;
extern crate winapi;

use widestring::WideCString;
use self::winapi::{c_int};
use sys::*;

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub const MONO_WIDTH: usize   = sys::MONO_WIDTH;
pub const MONO_HEIGHT: usize  = sys::MONO_HEIGHT;
pub const COLOR_WIDTH: usize  = sys::COLOR_WIDTH;
pub const COLOR_HEIGHT: usize = sys::COLOR_HEIGHT;

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

pub struct MonoLcd;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ColorButton {
    ButtonLeft,
    ButtonRight,
    ButtonOk,
    ButtonCancel,
    ButtonUp,
    ButtonDown,
    BttonMenu,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MonoButton {
    Button0,
    Button1,
    Button2,
    Button3,
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
        }
    }
}

impl Display for LcdError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "LcdError: {}", self.description())
    }
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
    pub fn connect(app_name: &str) -> Result<MonoLcd, LcdError> {
        assert_eq!(INITIALIZED.swap(true, Ordering::SeqCst), false);

        let ws = WideCString::from_str(app_name).unwrap();
        let ret = unsafe {
            match LogiLcdInit(ws.as_ptr(), LcdType::MONO) {
                Bool::TRUE => {
                    match LogiLcdIsConnected(LcdType::MONO) {
                        Bool::TRUE => Ok(MonoLcd),
                        Bool::FALSE => Err(LcdError::NotConnected),
                    }
                },
                Bool::FALSE => Err(LcdError::Initialization),
            }
        };
        if ret.is_err() {
            INITIALIZED.store(false, Ordering::SeqCst);
        }
        ret
    }

    pub fn is_connected() -> bool {
        unsafe {
            LogiLcdIsConnected(LcdType::MONO).into()
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

    pub fn set_background(&mut self, bytemap: &[u8]) -> Result<(), LcdError> {
        assert_eq!(bytemap.len(), MONO_WIDTH * MONO_HEIGHT);
        unsafe {
            match LogiLcdMonoSetBackground(bytemap.as_ptr()) {
                Bool::TRUE  => Ok(()),
                Bool::FALSE => Err(LcdError::MonoBackground),
            }
        }
    }

    pub fn set_text(&mut self, line_number: usize, text: &str) -> Result<(), LcdError> {
        let ws = WideCString::from_str(text).unwrap();
        assert!(line_number < 4);
        unsafe {
            match LogiLcdMonoSetText(line_number as c_int, ws.as_ptr()) {
                Bool::TRUE  => Ok(()),
                Bool::FALSE => Err(LcdError::MonoText),
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

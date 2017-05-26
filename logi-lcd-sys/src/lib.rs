#![allow(non_camel_case_types, non_snake_case)]

#[macro_use]
extern crate enumflags_derive;
extern crate enumflags;
extern crate winapi;
extern crate kernel32;
extern crate winreg;

pub use enumflags::{BitFlags, EnumFlagSize, InnerBitFlags};

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_CLASSES_ROOT, KEY_READ};
use winapi::minwindef::{HMODULE, FARPROC};
use winapi::winerror::{ERROR_MOD_NOT_FOUND, ERROR_PROC_NOT_FOUND};

use std::os::raw::{c_int, c_uint};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

pub const MONO_WIDTH:  usize = 160;
pub const MONO_HEIGHT: usize = 43;
pub const MONO_BYTES_PER_PIXEL: usize = 1;
pub const COLOR_WIDTH:  usize = 320;
pub const COLOR_HEIGHT: usize = 240;
pub const COLOR_BYTES_PER_PIXEL: usize = 4;

/// Bitflags for specifying LCD types, combine with [BitFlags](struct.BitFlags.html)
#[derive(EnumFlags, Copy, Clone, Debug)]
#[repr(u32)]
pub enum LcdType {
    MONO  = 0x00000001,
    COLOR = 0x00000002,
}

/// Bitflags for LCD Buttons, combine with [BitFlags](struct.BitFlags.html)
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

#[derive(Clone, Debug)]
pub enum ErrorKind {
    DLLNotFound,
    SymbolNotFound(&'static str),
    UnexpectedSystemError {
        error_code: u32,
        line: u32,
        function: &'static str,
        file: &'static str,
    },
}

#[derive(Clone, Debug)]
pub struct Error(pub ErrorKind, String);

pub struct LogitechLcd {
    pub LogiLcdInit: unsafe extern "C" fn(friendlyName: *const u16, lcdType: c_uint) -> bool,
    pub LogiLcdIsConnected: unsafe extern "C" fn(lcdType: c_uint) -> bool,
    pub LogiLcdIsButtonPressed: unsafe extern "C" fn(button: c_uint) -> bool,
    pub LogiLcdUpdate: unsafe extern "C" fn(),
    pub LogiLcdShutdown: unsafe extern "C" fn(),

    // Monochrome LCD functions
    pub LogiLcdMonoSetBackground: unsafe extern "C" fn(monoBitmap: *const u8) -> bool,
    pub LogiLcdMonoSetText: unsafe extern "C" fn(lineNumber: c_int, text: *const u16) -> bool,

    // Color LCD functions
    pub LogiLcdColorSetBackground: unsafe extern "C" fn(colorBitmap: *const u8) -> bool,
    pub LogiLcdColorSetTitle: unsafe extern "C" fn(text: *const u16, red: c_int, green: c_int,
        blue: c_int) -> bool,
    pub LogiLcdColorSetText: unsafe extern "C" fn(lineNumber: c_int, text: *const u16, red: c_int,
        green: c_int, blue: c_int) -> bool,

    //UDK functions, use this only if working with UDK
    pub LogiLcdColorSetBackgroundUDK: unsafe extern "C" fn(partialBitmap: *const u8,
        arraySize: c_int) -> c_int,
    pub LogiLcdColorResetBackgroundUDK: unsafe extern "C" fn() -> c_int,
    pub LogiLcdMonoSetBackgroundUDK: unsafe extern "C" fn(partialBitmap: *const u8,
        arraySize: c_int) -> c_int,
    pub LogiLcdMonoResetBackgroundUDK: unsafe extern "C" fn() -> c_int,
    handle: HMODULE,
}

unsafe impl std::marker::Send for LogitechLcd {}

impl Error {
    fn new(kind: ErrorKind) -> Error {
        let msg = match kind {
            ErrorKind::DLLNotFound => format!("Couldn't find LogitechLcd.dll"),
            ErrorKind::SymbolNotFound(ref symbol) => format!(
                "Unable to find symbol \"{}\" in LogitechLcd.dll",
                symbol
            ),
            ErrorKind::UnexpectedSystemError{error_code, line, function, file} => format!(
                "Unexpected system error, error_code:{}, function:\"{}\", file:\"{}\" line:{}",
                error_code,
                function,
                file,
                line
            )
        };
        Error(kind, msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.1[..]
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
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

// Find LogitechLcd.dll in windows registry using its CLSID
fn dll_path_clsid() -> Result<Vec<u16>, ErrorKind> {
    let hkcl = RegKey::predef(HKEY_CLASSES_ROOT);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let mut dll_path = None;

    #[cfg(target_arch = "x86_64")]
    {
        match hkcl.open_subkey_with_flags(
            "CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ)
        {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }

        match hklm.open_subkey_with_flags(
            "SOFTWARE\\Classes\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary",
            KEY_READ)
        {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }
    }

    #[cfg(target_arch = "x86")]
    {
        match hkcl.open_subkey_with_flags(
            "Wow6432Node\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ)
        {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }

        match hklm.open_subkey_with_flags(
            "SOFTWARE\\Classes\\Wow6432Node\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary",
            KEY_READ)
        {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }

        match hklm.open_subkey_with_flags(
            "SOFTWARE\\Wow6432Node\\Classes\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary",
            KEY_READ)
        {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }
    }

    match dll_path {
        // Convert to widestring and terminate with \0\0.
        Some(p) => Ok(OsStr::new(&p[..]).encode_wide().chain(Some(0)).collect::<Vec<u16>>()),
        None => Err(ErrorKind::DLLNotFound),
    }
}

unsafe fn load_lib() -> Result<HMODULE, ErrorKind> {
    match dll_path_clsid() {
        Ok(wide_path) => {
            let handle = kernel32::LoadLibraryW(wide_path.as_ptr());
            if handle.is_null() {
                let ecode = kernel32::GetLastError();
                if ecode != ERROR_MOD_NOT_FOUND {
                    return Err(ErrorKind::UnexpectedSystemError {
                        function: "LoadLibraryW",
                        error_code: ecode,
                        file: file!(),
                        line: line!(),
                    });
                }
                // Fallthrough on ERROR_MOD_NOT_FOUND
            } else {
                return Ok(handle);
            }
        },
        Err(e) => {
            match e {
                ErrorKind::DLLNotFound => {},
                _ => return Err(e),
            }
        },
    }

    // Convert to widestring and terminate with \0\0.
    let wide_name = OsStr::new("LogitechLcd.dll").encode_wide().chain(Some(0)).collect::<Vec<u16>>();
    let handle = kernel32::LoadLibraryW(wide_name.as_ptr());
    if handle.is_null() {
        let ecode = kernel32::GetLastError();
        if ecode == ERROR_MOD_NOT_FOUND {
            Err(ErrorKind::DLLNotFound)
        } else {
            Err(ErrorKind::UnexpectedSystemError {
                function: "LoadLibraryW",
                error_code: ecode,
                file: file!(),
                line: line!(),
            })
        }
    } else {
        Ok(handle)
    }
}


unsafe fn load_lib_symbols() -> Result<LogitechLcd, ErrorKind> {
    let handle = load_lib()?;

    let mut symbols = [
        ("LogiLcdInit\0",                    0 as FARPROC),
        ("LogiLcdIsConnected\0",             0 as FARPROC),
        ("LogiLcdIsButtonPressed\0",         0 as FARPROC),
        ("LogiLcdUpdate\0",                  0 as FARPROC),
        ("LogiLcdShutdown\0",                0 as FARPROC),
        ("LogiLcdMonoSetBackground\0",       0 as FARPROC),
        ("LogiLcdMonoSetText\0",             0 as FARPROC),
        ("LogiLcdColorSetBackground\0",      0 as FARPROC),
        ("LogiLcdColorSetTitle\0",           0 as FARPROC),
        ("LogiLcdColorSetText\0",            0 as FARPROC),
        ("LogiLcdColorSetBackgroundUDK\0",   0 as FARPROC),
        ("LogiLcdColorResetBackgroundUDK\0", 0 as FARPROC),
        ("LogiLcdMonoSetBackgroundUDK\0",    0 as FARPROC),
        ("LogiLcdMonoResetBackgroundUDK\0",  0 as FARPROC),
    ];

    for i in symbols.iter_mut() {
        i.1 = kernel32::GetProcAddress(handle, i.0.as_ptr() as *const i8);
        if i.1.is_null() {
            let ecode = kernel32::GetLastError();

            println!("ecode: {:?}", ecode);

            kernel32::FreeLibrary(handle);

            return if ecode == ERROR_PROC_NOT_FOUND {
                Err(ErrorKind::SymbolNotFound(&i.0[..(i.0.len() - 1)]))
            } else {
                Err(ErrorKind::UnexpectedSystemError{
                    function: "GetProcAddress",
                    error_code: ecode,
                    file: file!(),
                    line: line!(),
                })
            };
        }
    }

    Ok(LogitechLcd {
        LogiLcdInit:                    std::mem::transmute(symbols[0].1),
        LogiLcdIsConnected:             std::mem::transmute(symbols[1].1),
        LogiLcdIsButtonPressed:         std::mem::transmute(symbols[2].1),
        LogiLcdUpdate:                  std::mem::transmute(symbols[3].1),
        LogiLcdShutdown:                std::mem::transmute(symbols[4].1),
        LogiLcdMonoSetBackground:       std::mem::transmute(symbols[5].1),
        LogiLcdMonoSetText:             std::mem::transmute(symbols[6].1),
        LogiLcdColorSetBackground:      std::mem::transmute(symbols[7].1),
        LogiLcdColorSetTitle:           std::mem::transmute(symbols[8].1),
        LogiLcdColorSetText:            std::mem::transmute(symbols[9].1),
        LogiLcdColorSetBackgroundUDK:   std::mem::transmute(symbols[10].1),
        LogiLcdColorResetBackgroundUDK: std::mem::transmute(symbols[11].1),
        LogiLcdMonoSetBackgroundUDK:    std::mem::transmute(symbols[12].1),
        LogiLcdMonoResetBackgroundUDK:  std::mem::transmute(symbols[13].1),
        handle: handle,
    })
}

impl LogitechLcd {
    pub fn load() -> Result<LogitechLcd, Error> {
        unsafe {
            load_lib_symbols().map_err(|e| Error::new(e))
        }
    }
}

impl Drop for LogitechLcd {
    fn drop(&mut self) {
        unsafe {
            kernel32::FreeLibrary(self.handle);
        }
    }
}

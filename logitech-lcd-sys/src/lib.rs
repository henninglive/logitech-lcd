#![allow(non_camel_case_types, non_snake_case)]

#[macro_use]
extern crate bitflags;

use std::os::raw::{c_int, c_uint};

/// Monochrome screen, pixel witdh
pub const MONO_WIDTH:  usize = 160;

/// Monochrome screen, pixel hight
pub const MONO_HEIGHT: usize = 43;

/// Color screen, pixel witdh
pub const COLOR_WIDTH:  usize = 320;

/// Color screen, pixel hight
pub const COLOR_HEIGHT: usize = 240;

bitflags! {
    /// Targeted lcd type.
    ///
    /// This library allows you to target either Mono or Color devices
    pub struct LcdType: u32 {
        /// Mono color.
        const MONO =  0x00000001;
        /// 32bit RGBA color.
        const COLOR = 0b00000010;
        /// Either mono or color.
        const EITHER = Self::MONO.bits | Self::COLOR.bits;
    }
}

bitflags! {
    /// Lcd Button bitmap.
    pub struct LcdButton: u32 {
        const MONO_BUTTON_0 = 0x00000001;
        const MONO_BUTTON_1 = 0x00000002;
        const MONO_BUTTON_2 = 0x00000004;
        const MONO_BUTTON_3 = 0x00000008;
        const MONO_BUTTON = Self::MONO_BUTTON_0.bits |
                            Self::MONO_BUTTON_1.bits |
                            Self::MONO_BUTTON_2.bits |
                            Self::MONO_BUTTON_3.bits;

        const COLOR_BUTTON_LEFT   = 0x00000100;
        const COLOR_BUTTON_RIGHT  = 0x00000200;
        const COLOR_BUTTON_OK     = 0x00000400;
        const COLOR_BUTTON_CANCEL = 0x00000800;
        const COLOR_BUTTON_UP     = 0x00001000;
        const COLOR_BUTTON_DOWN   = 0x00002000;
        const COLOR_BUTTON_MENU   = 0x00004000;
        const COLOR_BUTTON = Self::COLOR_BUTTON_LEFT.bits |
                             Self::COLOR_BUTTON_RIGHT.bits |
                             Self::COLOR_BUTTON_OK.bits |
                             Self::COLOR_BUTTON_CANCEL.bits |
                             Self::COLOR_BUTTON_UP.bits |
                             Self::COLOR_BUTTON_DOWN.bits |
                             Self::COLOR_BUTTON_MENU.bits;
    }
}

/// LogitechLcd library.
///
/// Contains library symbols/functions as fields. Will unload library when dropped.
pub struct LogitechLcd {
    // Main functions
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

    // UDK functions, use this only if working with UDK
    pub LogiLcdColorSetBackgroundUDK: unsafe extern "C" fn(partialBitmap: *const u8,
        arraySize: c_int) -> c_int,
    pub LogiLcdColorResetBackgroundUDK: unsafe extern "C" fn() -> c_int,
    pub LogiLcdMonoSetBackgroundUDK: unsafe extern "C" fn(partialBitmap: *const u8,
        arraySize: c_int) -> c_int,
    pub LogiLcdMonoResetBackgroundUDK: unsafe extern "C" fn() -> c_int,

    /// Library handle, will be freed on drop
    _library: platform::Library,
}

unsafe impl std::marker::Send for LogitechLcd {}

#[cfg(not(target_os = "windows"))]
mod platform {
    use super::LogitechLcd;
    use std::io::Error;

    pub struct Library;

    impl LogitechLcd {
        pub fn load() -> Result<LogitechLcd, Error> {
            unimplemented!();
        }
    }
}

#[cfg(target_os = "windows")]
mod platform {
    extern crate winapi;
    extern crate kernel32;
    extern crate winreg;

    use super::LogitechLcd;

    use self::winreg::RegKey;
    use self::winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_CLASSES_ROOT, KEY_READ};
    use self::winapi::minwindef::{HMODULE, FARPROC};

    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    use std::io::Error;

    pub struct Library(HMODULE);

    const ERROR_MOD_NOT_FOUND: i32 = winapi::winerror::ERROR_MOD_NOT_FOUND as i32;

    /// Find `LogitechLcd.dll` in Windows registry using its CLSID
    fn dll_path_clsid() -> Result<Vec<u16>, Error> {
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
            None => Err(Error::from_raw_os_error(ERROR_MOD_NOT_FOUND)),
        }
    }

    unsafe fn load_lib() -> Result<HMODULE, Error> {
        match dll_path_clsid() {
            Ok(wide_path) => {
                let handle = kernel32::LoadLibraryW(wide_path.as_ptr());
                if handle.is_null() {
                    let error = Error::last_os_error();
                    let ecode = error.raw_os_error().unwrap();
                    // Fallthrough on ERROR_MOD_NOT_FOUND
                    if ecode != ERROR_MOD_NOT_FOUND {
                        return Err(error);
                    }
                } else {
                    return Ok(handle);
                }
            },
            Err(e) => {
                match e.raw_os_error() {
                    Some(ERROR_MOD_NOT_FOUND) => {},
                    _ => return Err(e),
                }
            },
        }

        // Convert to widestring and terminate with \0\0.
        let wide_name = OsStr::new("LogitechLcd.dll").encode_wide().chain(Some(0)).collect::<Vec<u16>>();
        let handle = kernel32::LoadLibraryW(wide_name.as_ptr());
        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(handle)
        }
    }

    impl LogitechLcd {
        /// Try to locate and load 'LogitechLcd.dll'.
        pub fn load() -> Result<LogitechLcd, Error> {
            use std::mem;

            unsafe {
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
                        let error = Error::last_os_error();
                        kernel32::FreeLibrary(handle);
                        return Err(error);
                    }
                }

                Ok(LogitechLcd {
                    LogiLcdInit:                    mem::transmute(symbols[0].1),
                    LogiLcdIsConnected:             mem::transmute(symbols[1].1),
                    LogiLcdIsButtonPressed:         mem::transmute(symbols[2].1),
                    LogiLcdUpdate:                  mem::transmute(symbols[3].1),
                    LogiLcdShutdown:                mem::transmute(symbols[4].1),
                    LogiLcdMonoSetBackground:       mem::transmute(symbols[5].1),
                    LogiLcdMonoSetText:             mem::transmute(symbols[6].1),
                    LogiLcdColorSetBackground:      mem::transmute(symbols[7].1),
                    LogiLcdColorSetTitle:           mem::transmute(symbols[8].1),
                    LogiLcdColorSetText:            mem::transmute(symbols[9].1),
                    LogiLcdColorSetBackgroundUDK:   mem::transmute(symbols[10].1),
                    LogiLcdColorResetBackgroundUDK: mem::transmute(symbols[11].1),
                    LogiLcdMonoSetBackgroundUDK:    mem::transmute(symbols[12].1),
                    LogiLcdMonoResetBackgroundUDK:  mem::transmute(symbols[13].1),
                    _library: Library(handle),
                })
            }
        }
    }

    impl Drop for Library {
        fn drop(&mut self) {
            unsafe {
                kernel32::FreeLibrary(self.0);
            }
        }
    }
}
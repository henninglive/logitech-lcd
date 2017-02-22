extern crate winreg;

use winreg::RegKey;
use winreg::enums::*;

use std::path::{Path, PathBuf};
use std::env;

/*
KNOWN CLSIDs with Logitech Gaming Software on windows 7 x64:
x64 HKEY_CLASSES_ROOT\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x86 HKEY_CLASSES_ROOT\Wow6432Node\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x64 HKEY_LOCAL_MACHINE\SOFTWARE\Classes\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x86 HKEY_LOCAL_MACHINE\SOFTWARE\Classes\Wow6432Node\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x86 HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\Classes\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
*/

// Try and find path to LogitechLcd.dll searching for its CLSID
// in the windows registry.
fn dll_path_clsid() -> String {
    assert!(cfg!(target_pointer_width = "64"), "Unsupported target platform");

    let hkcl = RegKey::predef(HKEY_CLASSES_ROOT);
    let key = hkcl.open_subkey_with_flags(
        "CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ)
        .expect("Unable to find registry key \
                 'KEY_CLASSES_ROOT\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary', \
                 please make sure 'Logitech Gaming Software' is installed or \
                 manually specify a library path with the 'LOGITECH_LCD' environment
                 variable or by overriding build script in a manifest file");

    let lib_path = key.get_value::<String, &str>("").expect(
        "The 'KEY_CLASSES_ROOT\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary' \
        registry key is missing the '(default)' subkey");

    let lib_dir_path = Path::new(&lib_path[..]).parent().unwrap();
    String::from(lib_dir_path.to_str().unwrap())
}


fn main() {
    assert!(cfg!(windows), "Unsupported target platform");

    let lib_dir_str = match env::var("LOGITECH_LCD_LIB_DIR") {
        Ok(val) => val,
        Err(_)  => dll_path_clsid(),
    };

    let lib_path = Path::new(&lib_dir_str[..]).join("LogitechLcd.dll");
    assert!(lib_path.is_file());
    assert_eq!(lib_path.file_name().unwrap(), "LogitechLcd.dll");

    if env::var("LOGITECH_LCD_COPY_OUT").is_ok() {
        let out_dir_str = env::var("OUT_DIR").unwrap();
        let mut out_lib_path = PathBuf::from(out_dir_str);

        assert!(out_lib_path.pop());
        assert!(out_lib_path.pop());
        assert!(out_lib_path.pop());

        out_lib_path.push(lib_path.file_name().unwrap());
        if !out_lib_path.exists() {
            std::fs::copy(lib_path, out_lib_path).unwrap();
        }
    }

    println!("cargo:rustc-link-search=native={}", lib_dir_str);
    println!("cargo:rustc-link-lib=LogitechLcd");
}
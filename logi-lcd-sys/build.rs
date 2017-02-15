extern crate winreg;

use winreg::RegKey;
use winreg::enums::*;
use std::path::Path;

/*
KNOWN CLSIDs with Logitech Gaming Software on windows 7 x64:
x64 HKEY_CLASSES_ROOT\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x86 HKEY_CLASSES_ROOT\Wow6432Node\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x64 HKEY_LOCAL_MACHINE\SOFTWARE\Classes\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x86 HKEY_LOCAL_MACHINE\SOFTWARE\Classes\Wow6432Node\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
x86 HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\Classes\CLSID\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\ServerBinary
*/

fn main() {
    assert!(cfg!(all(windows, target_pointer_width = "64")), "Unsupported target platform");

    // Try and find path to LogitechLcd.dll searching for its CLSID 
    // in the windows registry.
    let hkcl = RegKey::predef(HKEY_CLASSES_ROOT);
    let key = hkcl.open_subkey_with_flags(
        "CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ)
        .expect("Unable to find registry key \
                 'KEY_CLASSES_ROOT\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary', \
                 please make sure 'Logitech Gaming Software' is installed or \
                 manually specify SDK path by overriding this build script \
                 in any acceptable Cargo configuration location.");

    let dll_path_str = key.get_value::<String, &str>("").expect(
        "The 'KEY_CLASSES_ROOT\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary' \
        registry key is missing the '(default)' subkey");

    let dll_path = Path::new(&dll_path_str[..]);
    let dir_str = dll_path.parent().unwrap().to_str().unwrap();

    println!("cargo:rustc-link-search=native={}", dir_str);
    println!("cargo:rustc-link-lib=LogitechLcd");
}
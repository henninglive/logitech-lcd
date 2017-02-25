extern crate winreg;

use winreg::RegKey;
use winreg::enums::*;

use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

// Find LogitechLcd.dll in windows registry using its CLSID
fn dll_path_clsid(machine: &str) -> Option<String> {
    let hkcl = RegKey::predef(HKEY_CLASSES_ROOT);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let mut dll_path = None;

    if machine == "x64" {
        match hkcl.open_subkey_with_flags("CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ) {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }

        match hklm.open_subkey_with_flags("SOFTWARE\\Classes\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ) {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }
    } else {
        match hkcl.open_subkey_with_flags("Wow6432Node\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ) {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }

        match hklm.open_subkey_with_flags("SOFTWARE\\Classes\\Wow6432Node\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ) {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }

        match hklm.open_subkey_with_flags("SOFTWARE\\Wow6432Node\\Classes\\CLSID\\{d0e790a5-01a7-49ae-ae0b-e986bdd0c21b}\\ServerBinary", KEY_READ) {
            Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
            Err(_) => {},
        }
    }

    dll_path.map(|p| {
        let lib_dir_path = Path::new(&p[..]).parent().unwrap();
        String::from(lib_dir_path.to_str().unwrap())
    })
}

// Find visual studio install
fn vs_path() -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    match hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\VisualStudio\\SxS\\VS7", KEY_READ) {
        Ok(key) => {
            match key.get_value::<String, &str>("14.0") {
                Ok(s) => return Some(s),
                Err(_) => {},
            }
        },
        Err(_) => {},
    }

    match hklm.open_subkey_with_flags("SOFTWARE\\Wow6432Node\\Microsoft\\VisualStudio\\SxS\\VS7", KEY_READ) {
        Ok(key) => {
            match key.get_value::<String, &str>("14.0") {
                Ok(s) => return Some(s),
                Err(_) => {},
            }
        },
        Err(_) => {},
    }

    match hkcu.open_subkey_with_flags("SOFTWARE\\Wow6432Node\\Microsoft\\VisualStudio\\SxS\\VS7", KEY_READ) {
        Ok(key) => {
            match key.get_value::<String, &str>("14.0") {
                Ok(s) => return Some(s),
                Err(_) => {},
            }
        },
        Err(_) => {},
    }

    None
}

fn main() {
    assert!(cfg!(windows), "Unsupported target platform");

    let target_str = env::var("TARGET").unwrap();
    let out_dir_str = env::var("OUT_DIR").unwrap();

    let machine = if target_str.starts_with("x86_64") {
        "x64"
    } else if target_str.starts_with("i686") {
        "x86"
    } else {
        panic!("Unknown msvc target: {}", target_str);
    };

    // Get location of library from env var or try get path form winreg
    let lib_dir_str = match env::var("LOGITECH_LCD_LIB_DIR") {
        Ok(val) => val,
        Err(_)  => dll_path_clsid(machine).expect("Couldn't find the logitech lcd sdk. \
            Please make sure \"Logitech Gaming Software\" is installed or \
            manually specify a library path with the \"LOGITECH_LCD_LIB_DIR\" \
            environment variable"),
    };

    let dll_path = Path::new(&lib_dir_str[..]).join("LogitechLcd.dll");

    // MSVC requires both a .dll file and a .lib import library
    if target_str.contains("msvc") {
        let lib_path = Path::new(&lib_dir_str[..]).join("LogitechLcd.lib");

        // We are missing the a .lib import library
        if !lib_path.exists() {

            // Get visual studio install path
            let vs_path_str = vs_path().expect("Couldn't find visual studio install");

            let lib_tool_path = Path::new(&vs_path_str[..]).join("VC\\bin\\lib.exe");
            if !lib_tool_path.is_file() {
                panic!("Couldn't find lib.exe at \"{}\"", lib_tool_path.to_str().unwrap());
            }

            // Create .def file which contains a list symbols in our .dll file
            let def_file_path = Path::new(&out_dir_str[..]).join("LogitechLcd.def");
            let def_str = include_str!("LogitechLcd.def");
            let mut def_file = File::create(def_file_path).unwrap();
            def_file.write_all(def_str.as_bytes()).unwrap();

            // Use the VS lib tool to create a .lib import library for our .dll file
            Command::new(lib_tool_path)
                .arg("/def:LogitechLcd.def")
                .arg("/out:LogitechLcd.lib")
                .arg(format!("/machine:{}", machine))
                .current_dir(&out_dir_str)
                .spawn()
                .expect("Failed to create .lib import library using lib.exe");

            // Link to the .lib file we created
            println!("cargo:rustc-link-search=native={}", out_dir_str);
        } else {
            if !lib_path.is_file() {
                panic!("Couldn't find LogitechLcd.lib at \"{}\"", lib_path.to_str().unwrap());
            }

            // We found a .lib file, use that
            println!("cargo:rustc-link-search=native={}", lib_dir_str);
        }

    } else {
        // The gnu Compiler can link directly to our .dll file
        if !dll_path.is_file() {
            panic!("Couldn't find LogitechLcd.dll at \"{}\"", dll_path.to_str().unwrap());
        }

        // link to the .dll file
        println!("cargo:rustc-link-search=native={}", lib_dir_str);
    }

    if env::var("LOGITECH_LCD_COPY_OUT").is_ok() {
        if !dll_path.is_file() {
            panic!("Couldn't find LogitechLcd.dll at \"{}\"", dll_path.to_str().unwrap());
        }

        // Start with build out folder
        // Example: logi-lcd\target\debug\build\logi-lcd-sys-cb228bca7013f026\out
        let mut target_path = PathBuf::from(&out_dir_str);

        // Pop down to our traget dir: logi-lcd\target\debug
        assert!(target_path.pop());
        assert!(target_path.pop());
        assert!(target_path.pop());

        // Copy dll if doesn't exist
        target_path.push(dll_path.file_name().unwrap());
        if !target_path.exists() {
            std::fs::copy(dll_path, target_path).unwrap();
        }
    }

    println!("cargo:rustc-link-lib=LogitechLcd");
}
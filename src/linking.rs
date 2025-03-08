use libloading::{Library, Symbol};
use std::error::Error;

use crate::structs::ExtensionUI;

pub struct Extension {
    _library: Library,
    pub name: String,
    pub ui: Vec<ExtensionUI>,
}

fn chars_to_string(chars: *const std::ffi::c_char) -> Result<String, Box<dyn std::error::Error>> {
    if chars.is_null() {
        Ok("".to_string())
    } else {
        unsafe {
            match std::ffi::CStr::from_ptr(chars).to_str() {
                Ok(str) => Ok(str.to_owned()),
                Err(e) => Err(Box::new(e)),
            }
        }
    }
}

impl Extension {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        // Safely load the library
        let library = unsafe { Library::new(path)? };

        // Define how to extract and wrap the function

        unsafe {
            // Attempt to load the specified function from the library
            let get_ui: Symbol<unsafe extern "C" fn() -> *const std::ffi::c_char> =
                library.get(b"get_ui")?;

            let json = chars_to_string((get_ui)())?;

            let get_name: Symbol<unsafe extern "C" fn() -> *const std::ffi::c_char> =
                library.get(b"get_name")?;

            let name = chars_to_string((get_name)())?;

            Ok(Self {
                _library: library,
                name,
                ui: serde_json::from_str(&json)?,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_func() {
        let ext = Extension::load("/home/tpl/projects/susie/test.so").unwrap();
        println!("{:?}", ext.ui);
    }
}

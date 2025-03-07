use libloading::{Library, Symbol};
use std::error::Error;

use crate::structs::UIDescriptor;

pub struct Extension {
    _library: Library,
    inner_get_ui: unsafe extern "C" fn() -> *const std::ffi::c_char,
}

impl Extension {
    pub fn get_ui(&self) -> Result<Vec<UIDescriptor>, Box<dyn std::error::Error>> {
        let str = unsafe {
            let chars: *const std::ffi::c_char = (self.inner_get_ui)();

            if chars.is_null() {
                return Ok(vec![]);
            }

            match std::ffi::CStr::from_ptr(chars).to_str() {
                Ok(str) => str.to_owned(),
                Err(e) => return Err(Box::new(e)),
            }
        };

        Ok(serde_json::from_str(&str)?)
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

            let inner_get_ui = *get_ui;

            Ok(Extension {
                _library: library,
                inner_get_ui,
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
        println!("{:?}", ext.get_ui());
    }
}

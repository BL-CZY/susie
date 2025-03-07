use libloading::{Library, Symbol};
use std::error::Error;

use crate::structs::UIDescriptor;

pub struct Extension {
    _library: Library,
    pub ui: Vec<UIDescriptor>,
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

            let chars: *const std::ffi::c_char = (get_ui)();

            let str = {
                if chars.is_null() {
                    "".to_string()
                } else {
                    match std::ffi::CStr::from_ptr(chars).to_str() {
                        Ok(str) => str.to_owned(),
                        Err(e) => return Err(Box::new(e)),
                    }
                }
            };

            Ok(Self {
                _library: library,
                ui: serde_json::from_str(&str)?,
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

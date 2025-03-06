use libloading::{Library, Symbol};
use std::error::Error;

pub struct Extension {
    _library: Library,
    inner_get_ui: unsafe extern "C" fn() -> *const std::ffi::c_char,
}

impl Extension {
    pub fn get_ui(&self) -> Result<String, Box<dyn std::error::Error>> {
        unsafe {
            let chars: *const std::ffi::c_char = (self.inner_get_ui)();

            if chars.is_null() {
                return Ok("".into());
            }

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
        assert_eq!(ext.get_ui().unwrap(), "Hello!".to_string());
    }
}

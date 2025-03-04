use app::Susie;

pub mod app;

fn main() -> iced::Result {
    iced::application("A cool counter", Susie::update, Susie::view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .run()
}

//dynamic linking stuff
//use libloading::{Library, Symbol};
//use std::env;
//use std::error::Error;
//
//// Define a trait that represents the function signature we expect from dynamic libraries
//trait MathOperation {
//    fn calculate(&self, a: f64, b: f64) -> f64;
//}
//
//// Struct to manage dynamic library loading
//struct DynamicLibrary {
//    _library: Library,
//    operation: Box<dyn MathOperation>,
//}
//
//impl DynamicLibrary {
//    // Load a dynamic library and extract a specific function
//    fn load(path: &str, func_name: &str) -> Result<Self, Box<dyn Error>> {
//        // Safely load the library
//        let library = unsafe { Library::new(path)? };
//
//        // Define how to extract and wrap the function
//        unsafe {
//            // Attempt to load the specified function from the library
//            let func: Symbol<unsafe extern "C" fn(f64, f64) -> f64> =
//                library.get(func_name.as_bytes())?;
//
//            // Create a wrapper that implements the MathOperation trait
//            let operation = Box::new(FunctionWrapper(*func));
//
//            Ok(DynamicLibrary {
//                _library: library,
//                operation,
//            })
//        }
//    }
//
//    // Perform the calculation using the dynamically loaded function
//    fn calculate(&self, a: f64, b: f64) -> f64 {
//        self.operation.calculate(a, b)
//    }
//}
//
//// Wrapper to convert the raw function pointer to our trait
//struct FunctionWrapper(unsafe extern "C" fn(f64, f64) -> f64);
//
//impl MathOperation for FunctionWrapper {
//    fn calculate(&self, a: f64, b: f64) -> f64 {
//        unsafe { (self.0)(a, b) }
//    }
//}
//
//fn main() -> Result<(), Box<dyn Error>> {
//    // Example of dynamically loading a library
//    let library_path = match env::consts::OS {
//        "windows" => "math_operations.dll",
//        "macos" => "libmath_operations.dylib",
//        "linux" => "/home/tpl/projects/susie/libmath_operations.so",
//        _ => panic!("Unsupported operating system"),
//    };
//
//    // Try loading different operations
//    let add_lib = DynamicLibrary::load(library_path, "add")?;
//    let multiply_lib = DynamicLibrary::load(library_path, "multiply")?;
//
//    // Perform calculations using dynamically loaded functions
//    println!("Addition: 5 + 3 = {}", add_lib.calculate(5.0, 3.0));
//    println!(
//        "Multiplication: 5 * 3 = {}",
//        multiply_lib.calculate(5.0, 3.0)
//    );
//
//    Ok(())
//}

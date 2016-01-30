//! Rust bindings the Microsoft ChakraCore JavaScript engine. This gives a safe wrapper to the ChakraCore API.
//!
//! A sample program that returns `Hello world!` looks like this:
//!
//! ```
//! extern crate chakracore;
//! 
//! use chakracore::api::{JsRuntimeAttributes};
//! use chakracore::types::{JsValueTypes, StringAsWchar};
//! use chakracore::rust::Runtime;
//! 
//! fn main() {
//!     let runtime = Runtime::new(JsRuntimeAttributes::JsRuntimeAttributeNone).unwrap();
//!     let script = String::from("(()=>{return \'Hello world!\'; })()").to_wchar();
//!     let label = String::from("test").to_wchar();
//!     let result = runtime.run_script(script, 0, label).unwrap();
//!     let result_str = result.to_string().unwrap();
//!     println!("{}", result_str);
//! }
//! ```

extern crate libc;
extern crate encoding;

/// Raw ChakraCore API types and functions.
///
/// Everything in this module has one-to-one mapping with the ChakraCore API. For documentation on the API refer to the [JSRT reference](https://github.com/Microsoft/ChakraCore/wiki/JavaScript-Runtime-%28JSRT%29-Reference).
pub mod api;

/// Type conversion traits for ChakraCore types and their Rust equivalent.
pub mod types;

/// Rust like API for embedding ChakraCore.
pub mod rust;

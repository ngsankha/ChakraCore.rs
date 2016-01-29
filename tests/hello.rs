extern crate libc;
extern crate encoding;
extern crate chakracore;

use chakracore::chakra_api::{JsRuntimeAttributes};
use chakracore::types::{JsValueTypes, StringAsWchar};
use chakracore::rust::Runtime;
use libc::{c_void, wchar_t, size_t};
use std::slice;
use std::mem;
use std::ptr;

#[test]
fn hello() {
    let mut runtime = Runtime::new(JsRuntimeAttributes::JsRuntimeAttributeNone);
    let script = String::from("(()=>{return \'Hello world!\';})()").to_wchar();
    let label = String::from("test").to_wchar();
    let result = runtime.run_script(script, 0, label);
    let resultStr = result.to_string();
    assert_eq!(resultStr, "Hello world!");
}

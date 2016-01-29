extern crate libc;
extern crate encoding;
extern crate chakracore;

use chakracore::chakra_api;
use chakracore::types::JsValueTypes;
use libc::{c_void, wchar_t, size_t};
use std::slice;
use std::mem;
use std::ptr;
use encoding::all::UTF_16LE;
use encoding::{Encoding, EncoderTrap};

fn to_u16(s: &str) -> Vec<u16> {
    let mut v: Vec<u8> = UTF_16LE.encode(s, EncoderTrap::Strict).unwrap();
    v.push(0); v.push(0);
    let arr: &[u16] = unsafe { slice::from_raw_parts(v.as_ptr() as *const _, v.len() / 2) };
    arr.iter().cloned().collect()
}

#[test]
fn hello() {
    unsafe {
        let mut rt = chakra_api::JsRuntimeHandle(ptr::null_mut() as *mut c_void);
        let mut cx = chakra_api::JsContextRef(ptr::null_mut() as *mut c_void);
        let mut result = chakra_api::JsValueRef(ptr::null_mut() as *mut c_void);
        chakra_api::JsCreateRuntime(chakra_api::JsRuntimeAttributes::JsRuntimeAttributeNone, None, &mut rt);
        chakra_api::JsCreateContext(rt, &mut cx);
        chakra_api::JsSetCurrentContext(cx);
        let script = to_u16("(()=>{return \'Hello world!\';})()");
        let label = to_u16("test");
        chakra_api::JsRunScript(script.as_ptr() as *const wchar_t, 1, label.as_ptr() as *const wchar_t, &mut result);
        let resultStr = result.to_string();
        assert_eq!(resultStr, "Hello world!");
        chakra_api::JsSetCurrentContext(chakra_api::JsContextRef(ptr::null_mut() as *mut c_void));
        chakra_api::JsDisposeRuntime(rt);
    }
}
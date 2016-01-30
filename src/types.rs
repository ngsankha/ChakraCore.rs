use api::{JsValueRef, JsConvertValueToString, JsStringToPointer, JsErrorCode};
use std::ptr;
use std::slice;
use libc::{c_void, wchar_t, size_t};
use encoding::all::UTF_16LE;
use encoding::{Encoding, EncoderTrap};

pub trait JsValueTypes {
    fn to_string(&self) -> Result<String, JsErrorCode>;
}

pub trait StringAsWchar {
    fn to_wchar(&self) -> *const wchar_t;
}

impl JsValueTypes for JsValueRef {
    fn to_string(&self) -> Result<String, JsErrorCode> {
        let mut js_string = JsValueRef(ptr::null_mut() as *mut c_void);
        let status = unsafe { JsConvertValueToString(*self, &mut js_string) };
        match status {
            JsErrorCode::JsNoError => {
                let mut wstr: *const wchar_t = ptr::null_mut();
                let mut str_len: size_t = 0;
                let status = unsafe { JsStringToPointer(js_string, &mut wstr, &mut str_len) };
                match status {
                    JsErrorCode::JsNoError => Ok(unsafe { String::from_utf16_lossy(slice::from_raw_parts(wstr as *const u16, str_len)) }),
                    _ => Err(status)                
                }
            },
            _ => Err(status)
        }   
    }
}

impl StringAsWchar for String {
    fn to_wchar(&self) -> *const wchar_t {
        let mut v: Vec<u8> = UTF_16LE.encode(&self, EncoderTrap::Strict).unwrap();
        v.push(0); v.push(0);
        let arr: &[u16] = unsafe { slice::from_raw_parts(v.as_ptr() as *const _, v.len() / 2) };
        let str_u16: Vec<u16> = arr.iter().cloned().collect();
        str_u16.as_ptr() as *const wchar_t
    }
}

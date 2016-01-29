use chakra_api::{JsValueRef, JsConvertValueToString, JsStringToPointer};
use std::ptr;
use std::slice;
use libc::{c_void, wchar_t, size_t};

pub trait JsValueTypes {
    fn to_string(&self) -> String;
}

impl JsValueTypes for JsValueRef {
    fn to_string(&self) -> String {
        let mut js_string = JsValueRef(ptr::null_mut() as *mut c_void);
        unsafe { JsConvertValueToString(*self, &mut js_string) };
        let mut wstr: *const wchar_t = ptr::null_mut();
        let mut str_len: size_t = 0;
        unsafe { JsStringToPointer(js_string, &mut wstr, &mut str_len) };
        unsafe { String::from_utf16_lossy(slice::from_raw_parts(wstr as *const u16, str_len)) }
    }
}

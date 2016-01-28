extern crate libc;
extern crate encoding;

use libc::{c_void, wchar_t, size_t};
use std::ptr;
use std::slice;
use encoding::all::UTF_16LE;
use encoding::{Encoding, EncoderTrap};

type JsRuntimeHandle = *mut c_void;
type JsBackgroundWorkItemCallback = Option<unsafe extern "system" fn(callbackState: *mut c_void)>;
type JsThreadServiceCallback = Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackData: *mut c_void)>;
type JsRef = *mut c_void;
type JsContextRef = JsRef;
type JsValueRef = JsRef;
type JsSourceContext = usize;

type JsErrorCode = usize;

enum JsRuntimeAttributes {
	JsRuntimeAttributeNone
}


#[link(name = "ChakraCore")]
extern "system" {
	fn JsCreateRuntime(attributes: JsRuntimeAttributes, threadService: JsThreadServiceCallback, runtime: *mut JsRuntimeHandle) -> JsErrorCode;
	fn JsCreateContext(runtime: JsRuntimeHandle, newContext: *mut JsContextRef) -> JsErrorCode;
	fn JsSetCurrentContext(context: JsContextRef) -> JsErrorCode;
	fn JsRunScript(script: *const wchar_t, sourceContext: JsSourceContext, sourceUrl: *const wchar_t, result: *mut JsValueRef) -> JsErrorCode;
	fn JsConvertValueToString(value: JsValueRef, stringValue: *mut JsValueRef) -> JsErrorCode;
	fn JsStringToPointer(value: JsValueRef, stringValue: *mut *const wchar_t, stringLength: *mut size_t) -> JsErrorCode;
	fn JsDisposeRuntime(runtime: JsRuntimeHandle) -> JsErrorCode;
}

fn to_u16(s: &str) -> Vec<u16> {
	let mut v: Vec<u8> = UTF_16LE.encode(s, EncoderTrap::Strict).unwrap();
	v.push(0); v.push(0);
	let arr: &[u16] = unsafe { slice::from_raw_parts(v.as_ptr() as *const _, v.len() / 2) };
	arr.iter().cloned().collect()
}

fn main() {
	unsafe {
		let mut rt: JsRuntimeHandle = ptr::null_mut();
		let mut cx: JsContextRef = ptr::null_mut();
		let mut result: JsValueRef = ptr::null_mut();
		let JS_INVALID_REFERENCE: JsRef = ptr::null_mut();
		JsCreateRuntime(JsRuntimeAttributes::JsRuntimeAttributeNone, None, &mut rt);
		JsCreateContext(rt, &mut cx);
		JsSetCurrentContext(cx);
		let script = to_u16("(()=>{return \'Hello world!\';})()");
		let label = to_u16("test");
		JsRunScript(script.as_ptr() as *const wchar_t, 1, label.as_ptr() as *const wchar_t, &mut result);
		let mut resultJSString: JsValueRef = ptr::null_mut();
		JsConvertValueToString(result, &mut resultJSString);
		let mut resultWC: *const wchar_t = ptr::null_mut();
    	let mut stringLength: size_t = 0;
    	JsStringToPointer(resultJSString, &mut resultWC, &mut stringLength);
    	let resultStr = String::from_utf16_lossy(slice::from_raw_parts(resultWC as *const u16, stringLength));
    	println!("{}", resultStr);
    	JsSetCurrentContext(JS_INVALID_REFERENCE);
    	JsDisposeRuntime(rt);
	}
}

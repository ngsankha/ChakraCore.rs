use libc::{c_void, wchar_t, size_t};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsRuntimeHandle(pub *mut c_void);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsContextRef(pub *mut c_void);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JsValueRef(pub *mut c_void);

pub type JsBackgroundWorkItemCallback = Option<unsafe extern "system" fn(callbackState: *mut c_void)>;
pub type JsThreadServiceCallback = Option<unsafe extern "system" fn(callback: JsBackgroundWorkItemCallback, callbackData: *mut c_void)>;
pub type JsSourceContext = usize;

pub enum JsErrorCode {
    JsNoError = 0
}

pub enum JsRuntimeAttributes {
    JsRuntimeAttributeNone
}

#[link(name = "ChakraCore")]
extern "system" {
    pub fn JsCreateRuntime(attributes: JsRuntimeAttributes, threadService: JsThreadServiceCallback, runtime: *mut JsRuntimeHandle) -> JsErrorCode;
    pub fn JsCreateContext(runtime: JsRuntimeHandle, newContext: *mut JsContextRef) -> JsErrorCode;
    pub fn JsSetCurrentContext(context: JsContextRef) -> JsErrorCode;
    pub fn JsRunScript(script: *const wchar_t, sourceContext: JsSourceContext, sourceUrl: *const wchar_t, result: *mut JsValueRef) -> JsErrorCode;
    pub fn JsConvertValueToString(value: JsValueRef, stringValue: *mut JsValueRef) -> JsErrorCode;
    pub fn JsStringToPointer(value: JsValueRef, stringValue: *mut *const wchar_t, stringLength: *mut size_t) -> JsErrorCode;
    pub fn JsDisposeRuntime(runtime: JsRuntimeHandle) -> JsErrorCode;
}

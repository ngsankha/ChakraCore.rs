use chakra_api::{JsRuntimeHandle, JsContextRef, JsValueRef, JsCreateRuntime, JsCreateContext, JsSetCurrentContext, JsRuntimeAttributes, JsDisposeRuntime, JsSourceContext, JsRunScript};
use std::ptr;
use libc::{c_void, wchar_t};

pub struct Runtime {
    rt: JsRuntimeHandle,
    cx: JsContextRef
}

impl Runtime {
    pub fn new(attributes: JsRuntimeAttributes) -> Runtime {
        let mut rt = JsRuntimeHandle(ptr::null_mut() as *mut c_void);
        let mut cx = JsContextRef(ptr::null_mut() as *mut c_void);
        unsafe {
            JsCreateRuntime(attributes, None, &mut rt);
            JsCreateContext(rt, &mut cx);
            JsSetCurrentContext(cx);
        }
        Runtime { rt: rt, cx: cx }
    }

    pub fn rt(&self) -> JsRuntimeHandle {
        self.rt
    }

    pub fn cx(&self) -> JsContextRef {
        self.cx
    }

    pub fn run_script(&self, script: *const wchar_t, context: JsSourceContext, label: *const wchar_t) -> JsValueRef {
        let mut result = JsValueRef(ptr::null_mut() as *mut c_void);
        unsafe { JsRunScript(script, context, label, &mut result) };
        result
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            JsSetCurrentContext(JsContextRef(ptr::null_mut() as *mut c_void));
            JsDisposeRuntime(self.rt);
        }
    }
}

extern crate libc;
extern crate encoding;
extern crate chakracore;

use chakracore::chakra_api::{JsRuntimeAttributes};
use chakracore::types::{JsValueTypes, StringAsWchar};
use chakracore::rust::Runtime;

#[test]
fn hello() {
    let runtime = Runtime::new(JsRuntimeAttributes::JsRuntimeAttributeNone);
    let script = String::from("(()=>{return \'Hello world!\'; })()").to_wchar();
    let label = String::from("test").to_wchar();
    let result = runtime.run_script(script, 0, label);
    let result_str = result.to_string();
    assert_eq!(result_str, "Hello world!");
}

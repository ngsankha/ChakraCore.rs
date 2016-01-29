extern crate chakracore;

use chakracore::chakra_api::{JsRuntimeAttributes};
use chakracore::types::{JsValueTypes, StringAsWchar};
use chakracore::rust::Runtime;

#[test]
fn run_script() {
    let runtime = Runtime::new(JsRuntimeAttributes::JsRuntimeAttributeNone).unwrap();
    let script = String::from("(()=>{return \'Hello world!\'; })()").to_wchar();
    let label = String::from("test").to_wchar();
    let result = runtime.run_script(script, 0, label).unwrap();
    let result_str = result.to_string().unwrap();
    assert_eq!(result_str, "Hello world!");
}

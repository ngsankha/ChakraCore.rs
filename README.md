# ChakraCore.rs [![Build Status](https://ci.appveyor.com/api/projects/status/github/sankha93/ChakraCore.rs?svg=true)](https://ci.appveyor.com/project/sankha93/chakracore-rs)

Work in progress Rust bindings to [Microsoft ChakraCore](https://github.com/Microsoft/ChakraCore) JavaScript engine. Works on Windows only at the moment.

## Example program

The following program embeds ChakraCore and runs a JavaScript program that returns a string.

```Rust
extern crate chakracore;

use chakracore::chakra_api::{JsRuntimeAttributes};
use chakracore::types::{JsValueTypes, StringAsWchar};
use chakracore::rust::Runtime;

fn main() {
    let runtime = Runtime::new(JsRuntimeAttributes::JsRuntimeAttributeNone).unwrap();
    let script = String::from("(()=>{return \'Hello world!\'; })()").to_wchar();
    let label = String::from("test").to_wchar();
    let result = runtime.run_script(script, 0, label).unwrap();
    let result_str = result.to_string().unwrap();
    println!("{}", result_str);
}
```

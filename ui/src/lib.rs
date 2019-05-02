#[macro_use]
extern crate stdweb;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    let message = "Hello, Add!";
    js! {
        alert( @{message} );
    }

    a + b
}

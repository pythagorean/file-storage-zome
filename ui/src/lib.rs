#[macro_use]
extern crate stdweb;
extern crate base64;
extern crate js_sys;
extern crate web_sys;

mod app;
mod file_storage_zome_client;

pub use app::App;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

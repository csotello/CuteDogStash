use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function log(str) { console.log(str); }")]
extern "C" {
    pub fn log(str: String);
}

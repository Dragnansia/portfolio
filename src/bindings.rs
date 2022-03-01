use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "moveTo")]
    pub fn move_to(name: &str);
}

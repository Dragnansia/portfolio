use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "moveTo")]
    pub fn move_to(name: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "halfmoon.toggleModal")]
    pub fn toggle_modal(name: &str);
}

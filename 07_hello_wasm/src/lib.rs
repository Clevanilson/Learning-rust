use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(string: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// wasm-pack build --target web.
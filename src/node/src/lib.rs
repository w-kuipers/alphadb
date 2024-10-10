use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_js(param: &str) -> String {
    println!("Runs in JavaScript: {}", param);

    return format!("returned '{}' from Rust", param);
}

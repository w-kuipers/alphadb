use wasm_bindgen::prelude::*;
use alphadb::AlphaDB as AlphaDBCore;

#[wasm_bindgen]
struct AlphaDB {
    pub alphadb_instance: AlphaDBCore,
}

#[wasm_bindgen]
impl AlphaDb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            alphadb_instance: AlphaDBCore::new(),
        }
    }

    pub fn get(&self) -> i32 {
        self.internal
    }

    pub fn set(&mut self, val: i32) {
        self.internal = val;
    }
}

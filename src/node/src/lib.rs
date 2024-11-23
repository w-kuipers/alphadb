use wasm_bindgen::prelude::*;
use alphadb::AlphaDB as AlphaDBCore;

#[wasm_bindgen]
pub struct AlphaDB {
    pub alphadb_instance: AlphaDBCore,
}

#[wasm_bindgen]
impl AlphaDB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            alphadb_instance: AlphaDBCore::new(),
        }
    }

    // pub fn connect(
    //     &mut self,
    //     host: String,
    //     user: String,
    //     password: String,
    //     database: String,
    //     port: i32,
    // ) {
    //     self.alphadb_instance
    //         .connect(host, user, password, database, port)
    // }
}

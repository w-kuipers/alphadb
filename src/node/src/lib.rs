use alphadb::methods::connect::connect;
use mysql::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AlphaDB {
    connection: Option<PooledConn>,
    db_name: Option<String>,
}

#[wasm_bindgen]
impl AlphaDB {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            connection: None,
            db_name: None,
        }
    }

    pub fn connect(
        &mut self,
        host: String,
        user: String,
        password: String,
        database: String,
        port: u16,
    ) -> Result<(), JsValue> {
        // Establish connection to database
        let connection = connect(&host, &user, &password, &database, &port);

        match connection {
            Ok(connection) => {
                self.connection = Some(connection);
            }
            Err(e) => return Err(JsValue::from_str(e.to_string().as_str())),
        }

        // Set the database name
        self.db_name = Some(database.to_string());

        Ok(())
    }
}

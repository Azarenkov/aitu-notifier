use std::{env, error::Error, fs::File, io::Write};

use base64::{engine::general_purpose, Engine};

pub struct Config {
    pub service_account_key: String,
    pub kafka_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let service_account_key =
            env::var("SERVICE_ACCOUNT_KEY").expect("SERVICE_ACCOUNT_KEY must be set");
        let decoded_service_key = general_purpose::STANDARD
            .decode(service_account_key)
            .unwrap();
        let mut file = File::create("service_account_key.json")?;
        file.write_all(&decoded_service_key)?;

        Ok(Config {
            service_account_key: "service_account_key.json".to_string(),
            kafka_url: env::var("KAFKA_URL").expect("KAFKA_URL must be set"),
        })
    }
}

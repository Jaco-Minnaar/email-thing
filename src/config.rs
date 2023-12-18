use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub domain: String,
    pub server: String,
    pub email_addr: String,
    pub port: u16,
}

impl Config {
    pub fn new(domain: String, server: String, email_addr: String, port: u16) -> Self {
        Self {
            domain,
            server,
            email_addr,
            port,
        }
    }
}

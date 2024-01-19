use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub server: Server,
    pub log: Log,
    pub cert: Cert,
    pub jwt: Jwt,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub port: String,
    pub cors_allow_origin: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub filter_level: String,
    pub with_ansi: bool,
    pub to_stdout: bool,
    pub directory: String,
    pub file_name: String,
    pub rolling: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub jwt_secret: String,
    pub jwt_exp: i64,
}

#[derive(Debug, Deserialize)]
pub struct Cert {
    /// cert
    pub cert: String,
    /// key
    pub key: String,
}

const CONFIG_FILE: &str = "config/config.yml";

pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init);

impl Configs {
    pub fn init() -> Self {
        let mut file = match File::open(CONFIG_FILE) {
            Ok(f) => f,
            Err(e) => panic!(
                "Configuration file does not exist:{},error message:{}",
                CONFIG_FILE, e
            ),
        };
        let mut cfg_contents = String::new();
        match file.read_to_string(&mut cfg_contents) {
            Ok(s) => s,
            Err(e) => panic!("Failed to read configuration file, error message:{}", e),
        };
        match serde_yaml::from_str(&cfg_contents) {
            Ok(c) => c,
            Err(e) => panic!("Failed to parse configuration file, error message:{}", e),
        }
    }
}

use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub ip: String,
    pub port: i64
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Testing {
    pub testing: bool
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub server: Server,
    pub testing: Testing
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_env = env::var("ENV").unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            .add_source(File::with_name("./config/default"))
            .add_source(File::with_name(&format!("./config/{}", run_env)).required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_impl(){
        let test_env = Settings::new();

        assert!(!test_env.unwrap().testing.testing);
    }
}
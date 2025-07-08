use crate::model::main_models::ModelStrut;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tracing::info;

#[derive(Deserialize, Serialize, Debug)]
pub struct H1 {
    pub size: Option<u8>,
    pub conter: Option<bool>,
    pub context: Option<String>,
}

impl H1 {
    pub fn new() -> Self {
        info!("Init new struct h1");
        Self {
            size: None,
            conter: None,
            context: None,
        }
    }
}

impl ModelStrut for H1 {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<Self, anyhow::Error> {
        let load_file = fs::read_to_string(path)?;
        let h1 = toml::from_str::<H1>(&load_file)?;
        info!("Message: Load data: h1");
        Ok(h1)
    }

    fn write(&self) -> Result<()> {
        let data = H1::test();
        let context = toml::to_string_pretty(&data)?;
        fs::write("test/write/write.toml", context)?;
        Ok(())
    }

    fn test() -> Self {
        info!("create data test");
        let mut h1 = H1::new();
        h1.size = Some(16);
        h1.conter = Some(false);
        h1.context = Some("Hello Suomi".to_string());
        h1
    }
}

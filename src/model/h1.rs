use crate::model::main_models::ModelStrut;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tracing::{error, info};

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

    pub fn test(&mut self) -> &Self {
        info!("create data test");
        self.size = Some(16);
        self.conter = Some(false);
        self.context = Some("Hello Suomi".to_string());
        self
    }
}

impl ModelStrut for H1 {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let load_file = fs::read_to_string(path)?;
        let h1 = toml::from_str::<H1>(&load_file)?;
        info!("load data {:#?}", h1);
        Ok(())
    }

    fn write(&self) -> Result<()> {
        let mut data = H1::new();
        H1::test(&mut data);
        let context = toml::to_string_pretty(&data)?;
        let create = fs::write("test/write/write.toml", context);

        match create {
            Ok(_) => info!("Ok to write"),
            Err(e) => error!("{}", e),
        }
        Ok(())
    }
}

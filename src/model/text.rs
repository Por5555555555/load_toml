use crate::model::main_models::ModelStrut;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {
    pub context: Option<String>,
    pub type_text: Option<TypeText>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TypeText {
    H1,
    H2,
    H3,
    H4,
    H5,
    P,
}

impl Text {
    pub fn new() -> Self {
        Text {
            context: None,
            type_text: None,
        }
    }

    fn _check_size(&self) -> (u8, u8) {
        match &self.type_text {
            Some(TypeText::H1) => Text::_h1(),
            // H2 => info!("Type H2"),
            // H3 => info!("Type H3"),
            // H4 => info!("Type H4"),
            // H5 => info!("Type H5"),
            _ => (0, 0),
        }
    }

    fn _h1() -> (u8, u8) {
        info!("get h1");
        (0, 0)
    }
}

impl ModelStrut for Text {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<Self, anyhow::Error> {
        let load_file = fs::read_to_string(&path)?;
        let context = toml::from_str::<Text>(&load_file)?;
        info!("Message: Ok to load data");
        Ok(context)
    }
    fn write(&self) -> Result<()> {
        let data = Text::test();
        let context = toml::to_string_pretty(&data)?;
        fs::write("test/text/text.toml", context)?;
        Ok(())
    }
    fn test() -> Self {
        info!("Message: Create test Text");
        let mut text = Text::new();
        text.context = Some("Hello suomi".to_string());
        text.type_text = Some(TypeText::H1);
        text
    }
}

//
// impl ModelStrut for Text {
// fn load<P: AsRef<Path>, T>(&self, path: P) -> Result<&T> {
// let load_file = fs::read_to_string(path)?;
// let context = toml::from_str::<Text>(&load_file)?;
// info!("Message: Ok to load file : {:#?}", context);
// Ok(0)
// }
//
// fn write(&self) -> Result<()> {
// let data = Text::test();
// let context = toml::to_string_pretty(&data)?;
// let create = fs::write("test/text/text.toml", &context);
// info!("{} data: {:#?}", context, &data);
//
// match create {
// Ok(_) => info!("ok to write"),
// Err(e) => error!("{}", e),
// }
// Ok(())
// }
// }

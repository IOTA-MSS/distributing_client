use std::path::PathBuf;

use crate::library::app::AppDataBuilder;
use eyre::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub contract_address: String,
    pub node_url: String,
    pub database_path: String,
    pub chain_id: u16,
    pub fee: u32,
    pub server_address: String,
    pub bind_address: String,
}

impl ConfigFile {
    pub fn parse_to_app_builder(
        self,
        password: Option<String>,
        config_path: &str,
    ) -> eyre::Result<AppDataBuilder> {
        let database_path = {
            let mut database_path = PathBuf::from(config_path);
            database_path.pop();
            database_path.push(&self.database_path);
            database_path
        };

        Ok(AppDataBuilder {
            contract_address: self.contract_address,
            node_url: self.node_url,
            chain_id: self.chain_id,
            fee: self.fee,
            database_path: database_path.to_str().unwrap().to_owned(),
            password,
            server_address: self.server_address.parse()?,
            bind_address: self.bind_address.parse()?,
        })
    }

    pub fn from_path(path: &str) -> eyre::Result<Self> {
        Ok(toml::from_str(
            &std::fs::read_to_string(path)
                .wrap_err(format!("Config does not exist at path {:?}", path))?,
        )
        .wrap_err(format!("Could not parse config file at path {:?}", path))?)
    }
}
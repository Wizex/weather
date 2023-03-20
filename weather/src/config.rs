use crate::provider::Provider;
use anyhow::{Context, Error};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Result<T> = anyhow::Result<T>;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    provider_keys: HashMap<Provider, String>,
    selected_provider: Option<Provider>,
}

impl Config {
    pub fn load() -> Result<Self> {
        confy::load("weather", None)
            .map_err(Error::new)
            .with_context(|| "Failed to load a config")
    }

    pub fn store(&self) -> Result<()> {
        confy::store("weather", None, &self)
            .map_err(Error::new)
            .with_context(|| "Failed to store a config")
    }

    pub fn get_selected_provider_api_key(&self) -> Option<(Provider, Option<&String>)> {
        self.selected_provider
            .map(|provider| (provider, self.provider_keys.get(&provider)))
    }

    pub fn get_selected_provider(&self) -> Option<Provider> {
        self.selected_provider
    }

    pub fn set_api_key(&mut self, provider: Provider, key: String) {
        self.provider_keys.insert(provider, key);
    }

    pub fn set_selected_provider(&mut self, provider: Option<Provider>) {
        self.selected_provider = provider;
    }
}

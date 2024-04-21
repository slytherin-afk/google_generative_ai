mod builder;
mod generate_content;
mod get;
mod list;
mod model;

use reqwest::{Client, Url};
use std::sync::Arc;

pub use builder::*;
pub use generate_content::*;
pub use get::*;
pub use list::*;
pub use model::*;

#[derive(Clone)]
pub struct GenAiClient {
    inner: Arc<GenAiClientRef>,
}

struct GenAiClientRef {
    base_url: Url,
    fetch: Client,
}

impl GenAiClient {
    fn new(api_key: String) -> Self {
        GenAiClientBuilder::from(api_key)
            .build()
            .expect("GenAiClient::new()")
    }

    pub fn build(&self, model: &str) -> GenAiModel {
        GenAiModel {
            client: self.clone(),
            inner: Arc::new(GenAiModelRef {
                name: model.to_string(),
            }),
        }
    }
}

impl From<String> for GenAiClient {
    fn from(api_key: String) -> Self {
        GenAiClient::new(api_key)
    }
}

impl From<&str> for GenAiClient {
    fn from(api_key: &str) -> Self {
        GenAiClient::new(api_key.to_string())
    }
}

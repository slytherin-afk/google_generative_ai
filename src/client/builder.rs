use std::sync::Arc;

use anyhow::Context;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder, Url,
};

use super::{GenAiClient, GenAiClientRef};

pub struct GenAiClientBuilder {
    api_key: String,
    base_url: String,
}

impl GenAiClientBuilder {
    fn new(api_key: String) -> Self {
        let base_url = String::from("https://generativelanguage.googleapis.com/v1beta/models/");

        Self { api_key, base_url }
    }

    pub fn base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    pub fn build(self) -> anyhow::Result<GenAiClient> {
        let base_url = Url::parse(&self.base_url).context("failed to parse base url")?;

        let mut headers = HeaderMap::new();
        let api_key = HeaderValue::from_str(&self.api_key)
            .context("failed to convert api key to header value")?;

        headers.insert("x-goog-api-key", api_key);

        let fetch = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("failed to create reqwest::Client")?;

        let gen_ai_client = GenAiClient {
            inner: Arc::new(GenAiClientRef { base_url, fetch }),
        };

        Ok(gen_ai_client)
    }
}

impl From<String> for GenAiClientBuilder {
    fn from(api_key: String) -> Self {
        GenAiClientBuilder::new(api_key)
    }
}

impl From<&str> for GenAiClientBuilder {
    fn from(api_key: &str) -> Self {
        GenAiClientBuilder::new(api_key.to_string())
    }
}

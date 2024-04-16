use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{types::Model, GenAiRequestBuilder};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItemQueryParams {
    page_size: Option<u8>,
    page_token: Option<String>,
}

pub struct ListItem<'a> {
    request: &'a GenAiRequestBuilder,
    query_params: Option<&'a ListItemQueryParams>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListItemResponse {
    pub models: Vec<Model>,
    pub next_page_token: Option<String>,
}

impl<'a> ListItem<'a> {
    pub fn new(request: &'a GenAiRequestBuilder) -> Self {
        Self {
            request,
            query_params: None,
        }
    }

    pub fn query_params(self, query_params: &'a ListItemQueryParams) -> Self {
        Self {
            query_params: Some(&query_params),
            ..self
        }
    }

    pub async fn send(&self) -> Result<ListItemResponse, Box<dyn Error>> {
        let url: String = format!(
            "{}/{}/models",
            self.request.client.base_url, self.request.client.api_version
        );

        let fetch = &self.request.client.fetch;
        let response = fetch
            .get(url)
            .query(&self.query_params)
            .headers(self.request.client.headers.clone())
            .send()
            .await?;
        let result = response.json::<ListItemResponse>().await?;

        Ok(result)
    }
}

impl ListItemQueryParams {
    pub fn new() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }

    pub fn page_size(self, page_size: u8) -> Self {
        Self {
            page_size: Some(page_size),
            ..self
        }
    }

    pub fn page_token(self, page_token: &str) -> Self {
        Self {
            page_token: Some(page_token.to_string()),
            ..self
        }
    }
}

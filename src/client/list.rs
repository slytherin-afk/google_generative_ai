use anyhow::Context;
use serde::Serialize;

use super::GenAiModel;

use crate::ListResponse;

#[derive(Serialize)]
struct ListQueryParams {
    page_size: Option<u8>,
    page_token: Option<String>,
}

pub struct List {
    model: GenAiModel,
    query_params: ListQueryParams,
}

impl List {
    pub(crate) fn new(model: GenAiModel) -> Self {
        Self {
            model,
            query_params: ListQueryParams {
                page_size: None,
                page_token: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: u8) -> Self {
        self.query_params.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.query_params.page_token = Some(page_token);
        self
    }

    pub async fn send(self) -> anyhow::Result<ListResponse> {
        let url = self.model.client.inner.base_url.as_ref();
        let fetch = &self.model.client.inner.fetch;
        let response = fetch
            .get(url)
            .query(&self.query_params)
            .send()
            .await
            .context("request failed to get list of models")?;
        let result = response
            .json::<ListResponse>()
            .await
            .context("failed to deserialize get model list response body")?;

        Ok(result)
    }
}

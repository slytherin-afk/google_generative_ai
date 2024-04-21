use anyhow::Context;

use crate::{GenAiModel, GetResponse};

pub struct Get {
    model: GenAiModel,
}

impl Get {
    pub(crate) fn new(model: GenAiModel) -> Self {
        Self { model }
    }

    pub async fn send(self) -> anyhow::Result<GetResponse> {
        let url = self
            .model
            .client
            .inner
            .base_url
            .join(&self.model.inner.name)?;
        let fetch = &self.model.client.inner.fetch;
        let response = fetch
            .get(url)
            .send()
            .await
            .context("request failed to get model information")?;
        let result = response
            .json::<GetResponse>()
            .await
            .context("failed to deserialize get model response body")?;

        Ok(result)
    }
}

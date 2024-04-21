use crate::{types::Model, GenAiModel};

pub struct Get {
    model: GenAiModel,
}

type GetResponse = Model;

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
        let response = fetch.get(url).send().await?;
        let result = response.json::<GetResponse>().await?;

        Ok(result)
    }
}

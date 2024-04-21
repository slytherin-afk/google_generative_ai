use anyhow::Context;

use crate::{format_generate_content_input, GenerateContentRequest, GenerateContentResponse, Part};

use super::GenAiModel;

#[derive(Clone)]
pub enum ContentElement {
    Text(String),
    Part(Part),
}

#[derive(Clone)]
pub enum GenerateContentPrompt {
    Text(String),
    FullRequest(GenerateContentRequest),
    List(Vec<ContentElement>),
}

pub struct GenerateContent {
    model: GenAiModel,
}

impl GenerateContent {
    pub(crate) fn new(model: GenAiModel) -> Self {
        Self { model }
    }

    pub async fn send(
        self,
        prompt: GenerateContentPrompt,
    ) -> anyhow::Result<GenerateContentResponse> {
        let url = format!(
            "{}:generateContent",
            self.model
                .client
                .inner
                .base_url
                .join(&self.model.inner.name)
                .context("failed to parse url")?,
        );
        let formatted_params = format_generate_content_input(prompt)?;
        let fetch = &self.model.client.inner.fetch;
        let response = fetch
            .post(url)
            .json(&formatted_params)
            .send()
            .await
            .context("request to generate content failed")?;
        let result = response
            .json::<GenerateContentResponse>()
            .await
            .context("failed to deserialize generate content response body")?;

        Ok(result)
    }
}

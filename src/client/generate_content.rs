use crate::{format_generate_content_input, GenerateContentRequest, GenerateContentResponse, Part};

use super::GenAiModel;

#[derive(Clone)]
pub enum ContentElement {
    Text(String),
    Part(Part),
}

#[derive(Clone)]
pub enum GenerateContentInput {
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
        input: GenerateContentInput,
    ) -> anyhow::Result<GenerateContentResponse> {
        let formatted_params = format_generate_content_input(input)?;
        let url = self
            .model
            .client
            .inner
            .base_url
            .join(&(self.model.inner.name.to_owned() + "/generateContent"))?;
        let fetch = &self.model.client.inner.fetch;
        let response = fetch.get(url).send().await?;
        let result = response.json::<GenerateContentResponse>().await?;

        Ok(result)
    }
}

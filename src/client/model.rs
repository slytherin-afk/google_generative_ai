use std::sync::Arc;

use super::{GenAiClient, GenerateContent, Get, List};

#[derive(Clone)]
pub struct GenAiModel {
    pub(crate) client: GenAiClient,
    pub(crate) inner: Arc<GenAiModelRef>,
}

pub(crate) struct GenAiModelRef {
    pub name: String,
}

impl GenAiModel {
    pub fn get(&self) -> Get {
        Get::new(self.clone())
    }

    pub fn list(&self) -> List {
        List::new(self.clone())
    }

    pub fn generate_content(&self) -> GenerateContent {
        GenerateContent::new(self.clone())
    }
}

use serde::Serialize;

use super::{Content, GenerationConfig, SafetySetting, FunctionDeclarationsTool};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub tools: Option<Vec<FunctionDeclarationsTool>>,
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub generation_config: Option<GenerationConfig>,
}

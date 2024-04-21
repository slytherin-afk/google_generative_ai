use serde::{Deserialize, Serialize};

use super::{Content, HarmBlockThreshold, HarmCategory, Tool, ToolConfig};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub tools: Option<Vec<Tool>>,
    pub tool_config: Option<ToolConfig>,
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub system_instruction: Option<Content>,
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SafetySetting {
    category: HarmCategory,
    threshold: HarmBlockThreshold,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub stop_sequences: Option<Vec<String>>,
    pub candidate_count: Option<u8>,
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
}

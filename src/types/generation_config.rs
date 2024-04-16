use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub stop_sequences: Option<Vec<String>>,
    pub candidate_count: Option<u8>,
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<i32>,
    pub top_p: Option<i32>,
    pub top_k: Option<i32>,
}

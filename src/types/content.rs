use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Part {
    Text(TextPart),
    InlineData(InlineDataPart),
    FunctionCall(FunctionCallPart),
    FunctionResponse(FunctionResponsePart),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TextPart {
    pub text: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineDataPart {
    pub inline_data: GenerativeContentBlob,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallPart {
    pub function_call: FunctionCall,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponsePart {
    pub function_response: FunctionResponse,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FunctionCall {
    pub name: String,
    pub args: Value,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FunctionResponse {
    pub name: String,
    pub response: Value,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerativeContentBlob {
    pub mime_type: String,
    pub data: String,
}

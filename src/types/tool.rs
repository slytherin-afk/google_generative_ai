use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: Option<FunctionDeclarationSchema>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclarationSchema {
    #[serde(rename = "type")]
    pub type_: FunctionDeclarationSchemaType,
    pub format: Option<String>,
    pub description: Option<String>,
    pub nullable: Option<bool>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    pub properties: Option<HashMap<String, Box<FunctionDeclarationSchema>>>,
    pub required: Option<Vec<String>>,
    pub items: Box<FunctionDeclarationSchema>,
}

#[derive(Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum FunctionDeclarationSchemaType {
    #[serde(rename = "STRING")]
    #[strum(serialize = "STRING")]
    String,

    #[serde(rename = "NUMBER")]
    #[strum(serialize = "NUMBER")]
    Number,

    #[serde(rename = "INTEGER")]
    #[strum(serialize = "INTEGER")]
    Integer,

    #[serde(rename = "BOOLEAN")]
    #[strum(serialize = "BOOLEAN")]
    Boolean,

    #[serde(rename = "ARRAY")]
    #[strum(serialize = "ARRAY")]
    Array,

    #[serde(rename = "OBJECT")]
    #[strum(serialize = "OBJECT")]
    Object,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolConfig {
    function_calling_config: Option<FunctionCallingConfig>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallingConfig {
    mode: Option<Mode>,
    allowed_function_names: Option<Vec<String>>,
}

#[derive(Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum Mode {
    #[serde(rename = "AUTO")]
    #[strum(serialize = "AUTO")]
    Auto,

    #[serde(rename = "ANY")]
    #[strum(serialize = "ANY")]
    Any,

    #[serde(rename = "NONE")]
    #[strum(serialize = "NONE")]
    None,
}

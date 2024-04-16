use std::collections::HashMap;

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclarationsTool {
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: Option<FunctionDeclarationSchema>,
}

#[derive(Clone, Serialize)]
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

#[derive(Clone, strum_macros::Display, Serialize)]
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

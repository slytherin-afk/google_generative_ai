use serde::{Deserialize, Serialize};

#[derive(Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum FinishReason {
    #[serde(rename = "FINISH_REASON_UNSPECIFIED")]
    #[strum(serialize = "FINISH_REASON_UNSPECIFIED")]
    FinishReasonUnspecified,

    #[serde(rename = "STOP")]
    #[strum(serialize = "STOP")]
    Stop,

    #[serde(rename = "MAX_TOKENS")]
    #[strum(serialize = "MAX_TOKENS")]
    MaxTokens,

    #[serde(rename = "SAFETY")]
    #[strum(serialize = "SAFETY")]
    Safety,

    #[serde(rename = "RECITATION")]
    #[strum(serialize = "RECITATION")]
    Recitation,

    #[serde(rename = "OTHER")]
    #[strum(serialize = "OTHER")]
    Other,
}

#[derive(Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    #[strum(serialize = "HARM_CATEGORY_UNSPECIFIED")]
    HarmCategoryUnspecified,

    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    #[strum(serialize = "HARM_CATEGORY_DEROGATORY")]
    HarmCategoryDerogatory,

    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    #[strum(serialize = "HARM_CATEGORY_TOXICITY")]
    HarmCategoryToxicity,

    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    #[strum(serialize = "HARM_CATEGORY_VIOLENCE")]
    HarmCategoryViolence,

    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    #[strum(serialize = "HARM_CATEGORY_SEXUAL")]
    HarmCategorySexual,

    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    #[strum(serialize = "HARM_CATEGORY_MEDICAL")]
    HarmCategoryMedical,

    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    #[strum(serialize = "HARM_CATEGORY_DANGEROUS")]
    HarmCategoryDangerous,

    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    #[strum(serialize = "HARM_CATEGORY_HARASSMENT")]
    HarmCategoryHarassment,

    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    #[strum(serialize = "HARM_CATEGORY_HATE_SPEECH")]
    HarmCategoryHateSpeech,

    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    #[strum(serialize = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    HarmCategorySexuallyExplicit,

    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    #[strum(serialize = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    HarmCategoryDangerousContent,
}

#[derive(Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    #[strum(serialize = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    HarmBlockThresholdUnspecified,

    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    #[strum(serialize = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove,

    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    #[strum(serialize = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove,

    #[serde(rename = "BLOCK_ONLY_HIGH")]
    #[strum(serialize = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh,

    #[serde(rename = "BLOCK_NONE")]
    #[strum(serialize = "BLOCK_NONE")]
    BlockNone,
}

#[derive(Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    #[strum(serialize = "HARM_PROBABILITY_UNSPECIFIED")]
    HarmProbabilityUnspecified,

    #[serde(rename = "NEGLIGIBLE")]
    #[strum(serialize = "NEGLIGIBLE")]
    Negligible,

    #[serde(rename = "LOW")]
    #[strum(serialize = "LOW")]
    Low,

    #[serde(rename = "MEDIUM")]
    #[strum(serialize = "MEDIUM")]
    Medium,

    #[serde(rename = "HIGH")]
    #[strum(serialize = "HIGH")]
    High,
}

#[derive(Clone, strum_macros::Display, Deserialize, Serialize)]
pub enum BlockReason {
    #[serde(rename = "BLOCKED_REASON_UNSPECIFIED")]
    #[strum(serialize = "BLOCKED_REASON_UNSPECIFIED")]
    BlockedReasonUnspecified,

    #[serde(rename = "SAFETY")]
    #[strum(serialize = "SAFETY")]
    Safety,

    #[serde(rename = "OTHER")]
    #[strum(serialize = "OTHER")]
    Other,
}

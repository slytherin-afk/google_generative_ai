mod list;

pub use list::*;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

const DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com";
const DEFAULT_API_VERSION: &str = "v1beta";

#[derive(Clone)]
pub struct GenAiClient {
    fetch: Client,
    headers: HeaderMap,
    base_url: String,
    api_version: String,
}

pub struct GenAiRequestBuilder {
    client: GenAiClient,
    model: GenAiModel,
}

#[derive(strum_macros::Display)]
pub enum GenAiModel {
    #[strum(serialize = "gemini-pro-vision")]
    GeminiProVision,

    #[strum(serialize = "gemini-pro")]
    GeminiPro,
}

impl GenAiClient {
    /// Constructs a new `GenAiClient`.
    ///
    /// If the argument contains invalid header value characters, it will panic.
    /// Only visible ASCII characters (32-127) are permitted.
    pub fn new(api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        let api_key = HeaderValue::from_str(api_key).expect("Valid api key.");
        headers.insert("x-goog-api-key", api_key);

        let fetch = Client::new();

        Self {
            fetch,
            headers,
            base_url: DEFAULT_BASE_URL.to_string(),
            api_version: DEFAULT_API_VERSION.to_string(),
        }
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    pub fn api_version(mut self, version: &str) -> Self {
        self.api_version = version.to_string();
        self
    }

    pub fn model(&self, model: GenAiModel) -> GenAiRequestBuilder {
        GenAiRequestBuilder::new(self, model)
    }
}

impl GenAiRequestBuilder {
    fn new(client: &GenAiClient, model: GenAiModel) -> Self {
        GenAiRequestBuilder {
            client: client.clone(),
            model,
        }
    }

    pub fn list(&self) -> ListItem {
        ListItem::new(self)
    }
}

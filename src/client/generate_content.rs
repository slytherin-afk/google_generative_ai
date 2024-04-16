// use std::error::Error;

// use super::{GenAi, DEFAULT_API_VERSION, DEFAULT_URL};
// use reqwest::Client;
// use serde::{Deserialize, Serialize};

// impl GenAi {
//     pub async fn generate_content(&self) -> Result<(), Box<dyn Error>> {
//         let url = format!(
//             "{}/{}/{}:generateContent?key={}",
//             DEFAULT_URL, DEFAULT_API_VERSION, self.model, self.api_key
//         );

//         let client = Client::new();
//         // let response = client.post(url).query(&query_params).send().await?;
//         // let result = response.json::<ListResponse>().await?;

//         Ok(())
//     }
// }

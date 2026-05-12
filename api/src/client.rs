use reqwest::Client;
use crate::models::Container;
use crate::errors::ApiError;

pub struct WolframClient {
    app_id: String,
    http_client: Client,
}

impl WolframClient {
    pub fn new(app_id: String) -> Self {
        Self {
            app_id,
            http_client: Client::new(),
        }
    }

    pub async fn query(&self, input: &str) -> Result<Container, ApiError> {
        let url = "https://api.wolframalpha.com/v2/query";

        // Send the GET request with URL parameters
        let response = self.http_client.get(url)
            .query(&[
                ("input", input),
                ("appid", &self.app_id),
                ("output", "json"),
            ])
            .send()
            .await?;

        // Check if the HTTP status code is 200 OK
        if !response.status().is_success() {
            return Err(ApiError::Http(response.status().as_u16()));
        }

        // Deserialize the JSON payload into our structures
        let container = response.json::<Container>().await?;

        // Ensure Wolfram Alpha successfully parsed and computed the query
        if !container.result.success {
            return Err(ApiError::Wolfram("Wolfram Alpha returned success = false".to_string()));
        }

        Ok(container)
    }
}
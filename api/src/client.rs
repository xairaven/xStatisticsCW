use crate::errors::ApiError;
use crate::models::{Container, PodId};
use rand::distr::{Alphanumeric, SampleString};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
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

    pub async fn plain_text(&self, input: &str, pod: PodId) -> Result<String, ApiError> {
        let container = self.query(input).await?;

        let result = container
            .result
            .pods
            .iter()
            .find(|p| p.id == pod.to_string())
            .and_then(|pod| pod.sub_pods.first())
            .and_then(|sub_pod| sub_pod.plain_text.as_ref())
            .ok_or(ApiError::ImageNotFound)?;

        Ok(result.clone())
    }

    pub async fn short(&self, input: &str) -> Result<String, ApiError> {
        let url = "https://api.wolframalpha.com/v2/result";

        // Send the GET request with URL parameters
        let response = self
            .http_client
            .get(url)
            .query(&[("i", input), ("appid", &self.app_id)])
            .send()
            .await?;

        // Check if the HTTP status code is 200 OK
        if !response.status().is_success() {
            return Err(ApiError::Http(response.status().as_u16()));
        }

        let result = response.text().await?;

        Ok(result)
    }

    pub async fn image(&self, input: &str, pod: PodId) -> Result<PathBuf, ApiError> {
        let file_name = Alphanumeric.sample_string(&mut rand::rng(), 16);

        let save_path = self.save_path(&file_name).await?;

        let container = self.query(input).await?;

        let url = container
            .result
            .pods
            .iter()
            .find(|p| p.id == pod.to_string())
            .and_then(|pod| pod.sub_pods.first())
            .and_then(|sub_pod| sub_pod.img.as_ref())
            .map(|img| img.src.clone())
            .ok_or(ApiError::ImageNotFound)?;

        let response = self.http_client.get(&url).send().await?;

        // Ensure the request was successful before writing to disk
        if response.status().is_success() {
            let bytes = response.bytes().await?;
            let mut file = File::create(save_path.clone())?;
            file.write_all(&bytes)?;
            Ok(save_path)
        } else {
            Err(ApiError::Http(response.status().as_u16()))
        }
    }

    async fn query(&self, input: &str) -> Result<Container, ApiError> {
        let url = "https://api.wolframalpha.com/v2/query";

        // Send the GET request with URL parameters
        let response = self
            .http_client
            .get(url)
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
            return Err(ApiError::Wolfram(
                "Wolfram Alpha returned success = false".to_string(),
            ));
        }

        Ok(container)
    }

    async fn save_path(&self, file_name: &str) -> Result<PathBuf, ApiError> {
        let mut current_dir = std::env::current_exe().map_err(ApiError::IO)?;
        current_dir.pop(); // Remove executable name
        current_dir.push("temp");

        std::fs::create_dir_all(&current_dir).map_err(ApiError::IO)?;

        Ok(current_dir.join(file_name))
    }
}

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

const ANSWER_LIST: [PodId; 10] = [
    PodId::Result,
    PodId::ExpandedForm,
    PodId::AlternateForm,
    PodId::PolynomialForm,
    PodId::Solution,
    PodId::Value,
    PodId::Input,
    PodId::Integral,
    PodId::DefiniteIntegral,
    PodId::IndefiniteIntegral,
];

impl WolframClient {
    pub fn new(app_id: String) -> Self {
        Self {
            app_id,
            http_client: Client::new(),
        }
    }

    pub async fn plain_text(&self, input: &str, pod: PodId) -> Result<String, ApiError> {
        let container = self.query(input).await?;

        // Seeking for exact PodID
        let mut target_pod = container
            .result
            .pods
            .iter()
            .find(|p| p.id == pod.to_string());

        // If there's no exact PodID, search using a strict priority order.
        // "Input" is highly prioritized because trivial calculations (like Integrate(0))
        // often output their result directly in the Input block (e.g., "integral 0 dx = 0").
        if target_pod.is_none() {
            // Iterate through our priority list to grab the most relevant block first
            for id in ANSWER_LIST.iter() {
                if let Some(p) = container
                    .result
                    .pods
                    .iter()
                    .find(|p| p.id == id.to_string())
                {
                    target_pod = Some(p);
                    break;
                }
            }
        }

        // If not found in the priority list, get the very first block as a last resort
        if target_pod.is_none() {
            target_pod = container.result.pods.first();
        }

        match target_pod
            .and_then(|pod| pod.sub_pods.first())
            .and_then(|sub_pod| sub_pod.plain_text.as_ref())
        {
            None => {
                log::error!("Plain Text is not found in the response.");
                log::error!("Container: {:#?}", container);
                Err(ApiError::PlainTextNotFound)
            },
            Some(text) => Ok(text.to_string()),
        }
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

        // Same exact logic here for images
        let mut target_pod = container
            .result
            .pods
            .iter()
            .find(|p| p.id == pod.to_string());

        if target_pod.is_none() {
            for id in ANSWER_LIST.iter() {
                if let Some(p) = container
                    .result
                    .pods
                    .iter()
                    .find(|p| p.id == *id.to_string())
                {
                    target_pod = Some(p);
                    break;
                }
            }
        }

        if target_pod.is_none() {
            target_pod = container.result.pods.first();
        }

        let url = match target_pod
            .and_then(|pod| pod.sub_pods.first())
            .and_then(|sub_pod| sub_pod.img.as_ref())
            .map(|img| img.src.clone())
        {
            None => {
                log::error!("Image is not found in the response");
                log::error!("Container: {:#?}", container);
                return Err(ApiError::ImageNotFound);
            },
            Some(value) => value,
        };

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

    pub fn operand_from_result(&self, text: &str) -> String {
        // Defence from "where a = 1/10". If the string contains such condition,
        // we should not split it by `=`, otherwise we will lose the formula itself.
        if text.contains(" where ") {
            return text.trim().to_string();
        }

        // Split string by symbol "=" and get last part
        // If there's no "=", return whole string
        text.split('=')
            .next_back()
            .unwrap_or(text) // fallback (in case string is empty or doesn't contain "=")
            .trim()
            .to_string()
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

        let path = current_dir.join(file_name);

        log::info!("Save path: {}", path.to_string_lossy());

        Ok(path)
    }
}

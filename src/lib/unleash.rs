use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureResponse {
    pub name: String,
    pub enabled: bool,
    pub project: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturesResponse {
    pub version: u32,
    pub features: Vec<FeatureResponse>,
}

pub struct Client {
    url: String,
    token: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new(url: &str, token: &str) -> Client {
        let reqwest_client = ReqwestClient::new();
        Client {
            url: url.to_string(),
            token: token.to_string(),
            client: reqwest_client,
        }
    }

    pub async fn get_features(&self) -> Result<FeaturesResponse, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get(self.url.clone() + &"api/client/features".to_string())
            .header("Authorization", &self.token)
            .header("Accepts", "application/json")
            .send()
            .await?;

        let features = result.json::<FeaturesResponse>().await?;

        Ok(features)
    }
}

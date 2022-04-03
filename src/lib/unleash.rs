use serde::{Deserialize, Serialize};
use reqwest::{Client as ReqwestClient};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureResponse {
    name: String,
    enabled: bool,
    project: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturesResponse {
    version: u32,
    features: Vec<FeatureResponse>,
}

pub struct Client {
    url: String,
    token: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new(
        url: String,
        token: String,
    ) -> Client {
        let reqwest_client = ReqwestClient::new();
        Client {
            url,
            token,
            client: reqwest_client,
        }
    }

    pub async fn get_features(&self) -> Result<FeaturesResponse, Box<dyn std::error::Error>> {
        let result = self.client.get(self.url.clone() + &"api/client/features".to_string())
            .header("Authorization", &self.token)
            .header("Accepts", "application/json")
            .send()
            .await?;

        let features = result
            .json::<FeaturesResponse>()
            .await?;

        Ok(features)
    }
}
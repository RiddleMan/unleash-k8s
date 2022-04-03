use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::serde_json;
use k8s_openapi::serde_json::json;
use kube::api::PostParams;
use kube::{Api, Client};
use std::collections::HashMap;

pub struct K8s;

impl K8s {
    pub async fn save_secret(
        namespace: &str,
        name: &str,
        values: &HashMap<String, bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::try_default().await?;
        let secrets: Api<Secret> = Api::namespaced(client, namespace);

        let base64_values = values.iter().fold(HashMap::new(), |mut acc, (key, value)| {
            if *value {
                acc.insert(key.to_string(), "dHJ1ZQ==".to_string());
            } else {
                acc.insert(key.to_string(), "".to_string());
            }
            acc
        });
        let patch = serde_json::from_value(json!({
            "kind": "Secret",
            "apiVersion": "v1",
            "metadata": {
                "name": name
            },
            "type": "Opaque",
            "data": base64_values
        }))?;
        let res = secrets.create(&PostParams::default(), &patch).await;

        match res {
            Ok(_) => Ok(()),
            Err(_) => {
                secrets
                    .replace(name, &PostParams::default(), &patch)
                    .await?;
                Ok(())
            }
        }
    }
}

use std::collections::HashMap;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::serde_json;
use k8s_openapi::serde_json::json;
use kube::{Api, Client};
use kube::api::{Patch, PatchParams, PostParams};

struct K8s;

impl K8s {
    pub async fn save_secret(
        namespace: String,
        name: String,
        values: HashMap<String, bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::try_default().await?;
        let secrets: Api<Secret> = Api::namespaced(client, &namespace);


        let patch = serde_json::from_value(json!(values))?;
        secrets.replace(
            &name,
            &PostParams::default(),
            &patch,
        ).await?;

        Ok(())
    }
}

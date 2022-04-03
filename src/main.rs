mod lib;

use lib::unleash;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = lib::args::Args::parse();

    let features = unleash::Client::new(&args.unleash_url, &args.unleash_token)
        .get_features()
        .await?;

    let features_map = features
        .features
        .iter()
        .fold(HashMap::new(), |mut acc, feature| {
            let key = format!("{}.{}", feature.project, feature.name);
            acc.insert(key, feature.enabled);
            acc
        });

    lib::k8s::K8s::save_secret(&args.namespace, &args.secret_name, &features_map).await?;

    Ok(())
}

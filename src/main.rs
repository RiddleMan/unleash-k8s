mod lib;

use std::collections::HashMap;
use lib::unleash;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = lib::args::Args::parse();

    let client = unleash::Client::new(
        args.unleash_url,
        args.unleash_token,
    );
    let features = client.get_features().await?;

    let x = features.features.iter().fold(
        HashMap::new(),
        |mut acc, feature| {
            let key = format!("{}.{}", feature.project, feature.name);
            acc.insert(key, feature.enabled);
            acc
        }
    );

    println!("{:#?}", x);
    Ok(())
}


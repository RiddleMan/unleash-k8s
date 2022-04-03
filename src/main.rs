mod lib;

use lib::unleash;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = lib::args::Args::parse();

    let client = unleash::Client::new(
        args.unleash_url,
        args.unleash_token,
    );
    let features = client.get_features().await?;
    println!("{:#?}", features);
    Ok(())
}
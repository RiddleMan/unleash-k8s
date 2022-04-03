use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(long)]
    pub unleash_url: String,

    #[clap(long)]
    pub unleash_token: String,

    #[clap(long)]
    pub unleash_project: Option<String>,
}

pub struct Args {}

impl Args {
    pub fn parse() -> Arguments {
        let args = Arguments::parse();
        args
    }
}
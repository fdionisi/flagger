mod commands;

use clap::Parser;

use crate::commands::serve;

#[derive(Debug, Parser)]
#[clap(name = "flagger")]
#[clap(about = "Developer's tool to manage flagger services", long_about = None)]
enum FeatureFlagArgs {
    #[clap(subcommand)]
    Serve(serve::ServeArgs),
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    match FeatureFlagArgs::parse() {
        FeatureFlagArgs::Serve(serve) => serve::command(serve).await,
    }
}

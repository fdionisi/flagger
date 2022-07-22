use clap::Parser;
use flagger_api::ApiServer;
use flagger_core::Flagger;

#[derive(Debug, Parser)]
pub struct ServeApiArgs {
    #[clap(long)]
    api_port: u16,
    #[clap(long)]
    database_name: String,
    #[clap(long)]
    database_url: String,
}

pub async fn command(args: ServeApiArgs) {
    ApiServer::builder()
        .with_flagger(
            Flagger::builder()
                .with_database_url(args.database_url)
                .with_database_name(args.database_name)
                .build()
                .await
                .expect("msg"),
        )
        .build()
        .listen(("127.0.0.1", args.api_port))
        .await
        .expect("msg");
}

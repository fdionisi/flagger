#[cfg(any(feature = "rocksdb", feature = "sqlite"))]
use std::path::PathBuf;

use clap::Parser;
use flagger_consumer_service::ConsumerServer;
use flagger_core::Flagger;

#[cfg(feature = "mongodb")]
use flagger_mongodb_adapter::MongoDBAdapter;

#[cfg(feature = "rocksdb")]
use flagger_rocksdb_adapter::{RocksDBAdapter, RocksDBCompressionType, RocksDBOptions};

#[cfg(feature = "sqlite")]
use flagger_sqlite_adapter::SQLiteAdapter;

#[cfg(feature = "mongodb")]
#[derive(Debug, Parser)]
pub struct ServeApiArgs {
    #[clap(long)]
    api_address: String,
    #[clap(long)]
    api_port: u16,
    #[clap(long)]
    database_name: String,
    #[clap(long)]
    database_url: String,
}

#[cfg(feature = "sqlite")]
#[derive(Debug, Parser)]
pub struct ServeApiArgs {
    #[clap(long)]
    api_address: String,
    #[clap(long)]
    api_port: u16,
    #[clap(long)]
    database_path: PathBuf,
}

#[cfg(feature = "rocksdb")]
#[derive(Debug, Parser)]
pub struct ServeApiArgs {
    #[clap(long)]
    api_address: String,
    #[clap(long)]
    api_port: u16,
    #[clap(long)]
    database_dir: PathBuf,
    #[clap(long)]
    primary_dir: Option<PathBuf>,
}

pub async fn command(args: ServeApiArgs) {
    ConsumerServer::builder()
        .with_flagger(
            Flagger::builder()
                .with_database(database_adapter(&args).await)
                .build()
                .await
                .expect("msg"),
        )
        .build()
        .listen(
            format!("{}:{}", args.api_address, args.api_port)
                .parse()
                .unwrap(),
        )
        .await
        .expect("msg");
}

#[cfg(feature = "mongodb")]
async fn database_adapter(args: &ServeApiArgs) -> Box<MongoDBAdapter> {
    Box::new(
        MongoDBAdapter::builder()
            .with_database_url(&args.database_url)
            .with_database_name(&args.database_name)
            .build()
            .await,
    )
}

#[cfg(feature = "rocksdb")]
async fn database_adapter(args: &ServeApiArgs) -> Box<RocksDBAdapter> {
    let mut options = RocksDBOptions::default();
    options.create_if_missing(true);
    options.set_compression_type(RocksDBCompressionType::Snappy);

    Box::new(
        RocksDBAdapter::builder()
            .with_database_path(&args.database_dir)
            .with_options(options)
            .with_primary_path(args.primary_dir.clone())
            .build()
            .unwrap(),
    )
}

#[cfg(feature = "sqlite")]
async fn database_adapter(args: &ServeApiArgs) -> Box<SQLiteAdapter> {
    Box::new(
        SQLiteAdapter::builder()
            .with_database_path(&args.database_path)
            .build()
            .await,
    )
}

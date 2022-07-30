mod api;

use std::{ffi::OsString, path::Path, process::Stdio};

use clap::Parser;
use tokio::process::Command;

#[derive(Debug, Parser)]
pub enum ServeArgs {
    Api(api::ServeApiArgs),
    #[clap(external_subcommand)]
    Www(Vec<OsString>),
}

pub async fn command(args: ServeArgs) {
    match args {
        ServeArgs::Api(api_args) => api::command(api_args).await,
        ServeArgs::Www(args) => {
            let executable = Path::new("flagger-serve-www");

            Command::new(executable)
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .args(args.into_iter().skip(1).collect::<Vec<OsString>>())
                .spawn()
                .expect("fail to start")
                .wait()
                .await
                .expect("executable to be found");
        }
    }
}

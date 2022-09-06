mod api;
mod consumer;
mod management;

use std::{ffi::OsString, path::PathBuf, process::Stdio};

use clap::Parser;
use tokio::process::Command;

#[derive(Debug, Parser)]
pub enum ServeArgs {
    Api(api::ServeApiArgs),
    Consumer(consumer::ServeApiArgs),
    Management(management::ServeApiArgs),
    #[clap(external_subcommand)]
    External(Vec<OsString>),
}

pub async fn command(args: ServeArgs) {
    match args {
        ServeArgs::Api(api_args) => api::command(api_args).await,
        ServeArgs::Consumer(api_args) => consumer::command(api_args).await,
        ServeArgs::Management(api_args) => management::command(api_args).await,
        ServeArgs::External(mut args) => {
            let subcommand = args.remove(0);
            let executable = PathBuf::from(&format!(
                "flagger-serve-{}",
                subcommand.into_string().unwrap()
            ));
            println!("{:?}", &executable);

            Command::new(executable)
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .args(args.into_iter().collect::<Vec<OsString>>())
                .spawn()
                .expect("fail to start")
                .wait()
                .await
                .expect("executable to be found");
        }
    }
}

// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate onefuzz;

#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};
use std::io::{stdout, Write};

mod debug;
mod local;
mod managed;
mod tasks;

const LICENSE_CMD: &str = "licenses";
const LOCAL_CMD: &str = "local";
const DEBUG_CMD: &str = "debug";
const MANAGED_CMD: &str = "managed";

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let built_version = format!(
        "{} onefuzz:{} git:{}",
        crate_version!(),
        env!("ONEFUZZ_VERSION"),
        env!("GIT_VERSION")
    );

    let app = App::new("onefuzz-agent")
        .version(built_version.as_str())
        .subcommand(managed::cmd::args(MANAGED_CMD))
        .subcommand(local::cmd::args(LOCAL_CMD))
        .subcommand(debug::cmd::args(DEBUG_CMD))
        .subcommand(SubCommand::with_name(LICENSE_CMD).about("display third-party licenses"));

    let matches = app.get_matches();

    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run(matches))
}

async fn run(args: ArgMatches<'_>) -> Result<()> {
    match args.subcommand() {
        (LICENSE_CMD, Some(_)) => return licenses(),
        (DEBUG_CMD, Some(sub)) => return debug::cmd::run(sub).await,
        (LOCAL_CMD, Some(sub)) => return local::cmd::run(sub).await,
        (MANAGED_CMD, Some(sub)) => return managed::cmd::run(sub).await,
        _ => {
            anyhow::bail!("missing subcommand\nUSAGE: {}", args.usage());
        }
    }
}

fn licenses() -> Result<()> {
    stdout().write_all(include_bytes!("../../data/licenses.json"))?;
    Ok(())
}

use clap::Parser;

use diffreq::{Action, Args, ConfigLoad, GetProfile, RequestConfig, RunArgs};
use std::io::{self, Write};
use string_builder::Builder;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_args = Args::parse();
    match cli_args.action {
        Action::Run(run_args) => run(run_args).await?,
        Action::Parse => parse_profile().await?,
        _ => Err(anyhow::anyhow!("unknown action"))?,
    };
    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    //      如果没有值得则使用默认的xdiff.yml
    let config = args.config.unwrap_or_else(|| "./xreq.yml".to_string());
    let profile_name = args.profile;
    let config_profile = RequestConfig::load_yaml(&config).await?;
    let req = config_profile.get_profile(&profile_name).ok_or_else(|| {
        anyhow::anyhow!("Profile: {} not found in config: {}", profile_name, config)
    })?;
    let extra_args = args.extra_params.into();
    let _res = req.send(&extra_args).await?;

    // get res header and body text
    let output_builder = Builder::default();
    let mut stdout = io::stdout().lock();
    stdout.write_all(output_builder.string()?.as_bytes())?;
    // print to stdout

    Ok(())
}

async fn parse_profile() -> Result<()> {
    todo!()
}

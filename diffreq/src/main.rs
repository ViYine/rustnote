// pub mod config;
use anyhow::{anyhow, Result};
use clap::Parser;
use diffreq::{
    cli::{Action, Args, RunArgs},
    DiffConfig,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_args = Args::parse();
    match cli_args.action {
        Action::Run(run_args) => run(run_args).await?,
        _ => Err(anyhow!("unknown action"))?,
    };
    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    //      如果没有值得则使用默认的xdiff.yml
    let config = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let profile_name = args.profile;
    let config_profile = DiffConfig::load_yaml(&config).await?;
    let profile = config_profile.get_profile(&profile_name).ok_or_else(|| {
        anyhow::anyhow!("Profile: {} not found in config: {}", profile_name, config)
    })?;
    let extra_args = args.extra_params.into();
    profile.diff(extra_args).await?;
    Ok(())
}

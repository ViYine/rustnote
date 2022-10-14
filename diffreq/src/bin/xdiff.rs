use clap::Parser;
use dialoguer::{theme, Input, MultiSelect};
use diffreq::{
    util::hightlight_text, Action, Args, ConfigLoad, DiffConfig, DiffProfile, ExtraArgs,
    GetProfile, RequestProfile, ResponseProfile, RunArgs,
};
use std::io::{self, Write};

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

async fn parse_profile() -> Result<()> {
    //  交互式地生成profile
    let theme = theme::ColorfulTheme::default();
    let url1: String = Input::with_theme(&theme)
        .with_prompt("Url1")
        .interact_text()?;
    let url2: String = Input::with_theme(&theme)
        .with_prompt("Url2")
        .interact_text()?;

    // RequestProfile from String
    // RequestProfile 需要实现FromStr
    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;

    let profile_name: String = Input::with_theme(&theme)
        .with_prompt("Profile")
        .interact_text()?;

    let response1 = req1.send(&ExtraArgs::default()).await?;
    let headers_key = response1.get_header_keys();
    let chosen = MultiSelect::with_theme(&theme)
        .with_prompt("Select headers to skip")
        .items(&headers_key)
        .interact()?;
    let skip_headers: Vec<String> = chosen.iter().map(|i| headers_key[*i].to_string()).collect();

    // response profile contract
    // todo: implement skip_body
    let res: ResponseProfile = ResponseProfile::new(skip_headers, vec![]);
    let profile: DiffProfile = DiffProfile::new(req1, req2, res);
    // config
    let config: DiffConfig = DiffConfig::new(vec![(profile_name, profile)].into_iter().collect());
    let result = serde_yaml::to_string(&config)?;

    // output to stdout
    let mut std = std::io::stdout().lock();
    write!(
        std,
        "---\n{}",
        hightlight_text(&result, "yaml", "base16-ocean.dark")?
    )?;

    // println!("prase_profile..., {} ,{}, {}", url1, url2, profile);
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
    let diff_text = profile.diff(extra_args).await?;
    let mut stdout = io::stdout().lock();
    stdout.write_all(diff_text.as_bytes())?;
    // print to stdout

    Ok(())
}

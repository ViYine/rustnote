use clap::Parser;
use dialoguer::{theme, Input};
use diffreq::{
    get_body_text, get_header_text, get_status_text, util::hightlight_text, Action, Args,
    ConfigLoad, GetProfile, RequestConfig, RequestProfile, RunArgs,
};
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
    let res = req.send(&extra_args).await?.into_inner();

    let status_text = get_status_text(&res)?;
    let header_text = get_header_text(&res, &[])?;
    let body = get_body_text(res, &[]).await?;

    // get res header and body text
    let mut output_builder = Builder::default();

    output_builder.append(format!(
        "{}{}",
        hightlight_text(&status_text, "yaml", "Solarized (light)")?,
        hightlight_text(&header_text, "yaml", "Solarized (dark)")?
    ));
    output_builder.append(format!(
        "{}\n",
        hightlight_text(&body, "json", "base16-ocean.dark")?
    ));

    let mut stdout = io::stdout().lock();
    stdout.write_all(output_builder.string()?.as_bytes())?;
    // print to stdout

    Ok(())
}

async fn parse_profile() -> Result<()> {
    //  交互式地生成profile
    let theme = theme::ColorfulTheme::default();
    let url: String = Input::with_theme(&theme)
        .with_prompt("Url")
        .interact_text()?;

    // RequestProfile from String
    // RequestProfile 需要实现FromStr
    let req: RequestProfile = url.parse()?;

    let profile_name: String = Input::with_theme(&theme)
        .with_prompt("Profile")
        .interact_text()?;

    // config
    let config: RequestConfig = RequestConfig::new(vec![(profile_name, req)].into_iter().collect());
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

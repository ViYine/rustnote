use anyhow::Result;
use diffreq::{ConfigLoad, DiffConfig};

fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yml");
    let config = DiffConfig::from_yaml(content).await?;
    println!("{:#?}", config);
    Ok(())
}

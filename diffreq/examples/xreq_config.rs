use anyhow::Result;
use diffreq::{ConfigLoad, RequestConfig};

fn main() -> Result<()> {
    let content = include_str!("../fixtures/xreq.yml");
    let config = RequestConfig::from_yaml(content)?;
    println!("{:#?}", config);
    Ok(())
}

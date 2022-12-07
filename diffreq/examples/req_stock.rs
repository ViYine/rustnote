use serde::{Deserialize, Serialize};
use serde_json::value;

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpsMap {
    pub is_show: bool,
    pub rps10: Vec<Rps>,
    #[serde(rename = "rps10_today")]
    pub rps10_today: f64,
    pub rps120: Vec<Rps>,
    #[serde(rename = "rps120_today")]
    pub rps120_today: f64,
    pub rps20: Vec<Rps>,
    #[serde(rename = "rps20_today")]
    pub rps20_today: f64,
    pub rps250: Vec<Rps>,
    #[serde(rename = "rps250_today")]
    pub rps250_today: f64,
    pub rps50: Vec<Rps>,
    #[serde(rename = "rps50_today")]
    pub rps50_today: f64,
    pub rps60: Vec<Rps>,
    #[serde(rename = "rps60_today")]
    pub rps60_today: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rps {
    pub x: u64,
    pub y: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;

    // Perform the actual execution of the network request
    let res = client
        .get("https://www.zsxg.cn/api/v2/capital/info?code=600039.SH&yearNum=2")
        .send()
        .await?;

    // Parse the response body as Json in this case
    let value: value::Value = serde_json::from_str(&res.text().await?)?;

    // print the comment_new field
    let comment_new: String = value["datas"]["comment_new"].to_string();
    println!("comment_new: {}\n", comment_new);
    // print the segments field
    let segments_str: String = value["datas"]["segments"].to_string();
    println!("segments: {}\n", segments_str);
    // print the boll field
    let boll_str: String = value["datas"]["boll"].to_string();
    println!("boll: {}\n", boll_str);
    // print the briefing field
    let briefing_str: String = value["datas"]["briefing"].to_string();
    println!("briefing: {}\n", briefing_str);
    // print the indexList field
    let index_list_str: String = value["datas"]["indexList"].to_string();
    println!("indexList: {}\n", index_list_str);

    // // let rps_map: RpsMap = serde_json::from_value(value["datas"]["rpsMap"]);
    // // get rps_map for value["datas"]["rpsMap"]
    // let rps_map: RpsMap = serde_json::from_value(value["datas"]["rpsMap"].clone())?;
    // // println!("{}", serde_json::to_string(&rps_map.rps10)?);
    // // get x for rps10
    // let x_vec: Vec<String> = rps_map.rps20.iter().map(|rps| get_time_str(rps.x / 1000)).collect();
    // // get y for rps10
    // let y_vec: Vec<f64> = rps_map.rps20.iter().map(|rps| rps.y).collect();
    // println!("{:?}", x_vec);
    // println!("{:?}", y_vec);
    Ok(())
}

fn get_time_str(val: u64) -> String {
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(val);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y%m%d").to_string();
    timestamp_str
}

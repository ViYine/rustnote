use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use color_eyre::eyre::{eyre, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use tracing::{error, info};

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

#[tokio::main]
async fn main() -> Result<()> {
    // Build the client using the builder pattern
    tracing_subscriber::fmt::init();

    let sh_codes = include_str!("../fixtures/SH.code")
        .lines()
        .map(|s| s.to_string())
        // .map(|s| format!("SH{}", s.split('.').next().unwrap()))
        .collect::<Vec<String>>();
    let sz_codes = include_str!("../fixtures/SZ.code")
        .lines()
        .map(|s| s.to_string())
        // .map(|s| format!("SZ{}", s.split('.').next().unwrap()))
        .collect::<Vec<String>>();

    // merge sh and sz codes
    // let mut codes = sh_codes;
    // codes.extend(sz_codes);
    let cur_t = get_cur_time()?;
    info!(
        "start download all codes, total: {}, cur_time:{}",
        sz_codes.len() + sh_codes.len(),
        cur_t.as_secs()
    );

    download_parallel(sz_codes[..1000].to_vec()).await?;

    info!(
        "download 1000 sz_codes, cur_time: {}",
        get_cur_time()?.as_secs()
    );

    download_parallel(sh_codes[..1000].to_vec()).await?;

    info!(
        "download 1000 sh_codes, cur_time: {}",
        get_cur_time()?.as_secs()
    );

    download_parallel(sz_codes[1000..].to_vec()).await?;

    info!(
        "download all sz_codes, cur_time: {}",
        get_cur_time()?.as_secs()
    );

    download_parallel(sh_codes[1000..].to_vec()).await?;

    info!(
        "download all sh_codes, cur_time: {}",
        get_cur_time()?.as_secs()
    );

    // wait all tasks finish
    // calculate time cost
    info!("time cost: {}", get_cur_time()?.as_secs() - cur_t.as_secs());

    Ok(())
}

fn get_cur_time() -> Result<Duration> {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)?;
    Ok(since_the_epoch)
}

async fn download_parallel(codes: Vec<String>) -> Result<()> {
    // all codes download use function: fetch_income_data, fetch_asset_data, fetch_cash_data
    // use tokio::spawn to download codes in parallel
    // Retry up to 3 times with increasing intervals between attempts.
    let retry_policy = ExponentialBackoff::builder()
        .backoff_exponent(3)
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(60))
        .build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    // let client = reqwest::Client::builder().build()?;
    let futures: Vec<_> = codes
        .into_iter()
        .map(|code| {
            let client = client.clone();
            tokio::spawn(async move {
                let incode_data_file = format!("finial_data/{}_income.csv", code);
                let metadata = fs::metadata(&incode_data_file);
                
                if metadata.is_ok() {
                    let metadata = metadata.unwrap();
                    if metadata.modified()?.elapsed()?.as_secs() < 3600 && metadata.is_file() {
                        info!("file: {} already exists, skip", incode_data_file);
                    } else {
                        info!("file: {} not exists, download", incode_data_file);
                        let income_data = fetch_income_data(&client, &code).await?;
                        income_data.to_csv_file(&format!("finial_data/{}_income.csv", code))?;
                    }
                } else {
                    info!("file: {} not exists, download", incode_data_file);
                    let income_data = fetch_income_data(&client, &code).await?;
                    income_data.to_csv_file(&format!("finial_data/{}_income.csv", code))?;
                } 

                let asset_data_file = format!("finial_data/{}_asset.csv", code);
                let metadata = fs::metadata(&asset_data_file);
                if metadata.is_ok() {
                    let metadata = metadata.unwrap();
                    if metadata.modified()?.elapsed()?.as_secs() < 3600 && metadata.is_file() {
                        info!("file: {} already exists, skip", asset_data_file);
                    } else {
                        info!("file: {} not exists, download", asset_data_file);
                        let asset_data = fetch_asset_data(&client, &code).await?;
                        asset_data.to_csv_file(&format!("finial_data/{}_asset.csv", code))?;
                    }
                } else {
                    info!("file: {} not exists, download", asset_data_file);
                    let asset_data = fetch_asset_data(&client, &code).await?;
                    asset_data.to_csv_file(&format!("finial_data/{}_asset.csv", code))?;
                }

                let cash_data_file = format!("finial_data/{}_cash.csv", code);
                let metadata = fs::metadata(&cash_data_file);
                if metadata.is_ok() {
                    let metadata = metadata.unwrap();
                    if metadata.modified()?.elapsed()?.as_secs() < 3600 && metadata.is_file() {
                        info!("file: {} already exists, skip", cash_data_file);
                    } else {
                        info!("file: {} not exists, download", cash_data_file);
                        let cash_data = fetch_cash_data(&client, &code).await?;
                        cash_data.to_csv_file(&format!("finial_data/{}_cash.csv", code))?;
                    }
                } else {
                    info!("file: {} not exists, download", cash_data_file);
                    let cash_data = fetch_cash_data(&client, &code).await?;
                    cash_data.to_csv_file(&format!("finial_data/{}_cash.csv", code))?;
                }
                
                Ok::<(), color_eyre::eyre::Error>(())
            })
        })
        .collect();

    // do these futures in parallel and return them
    for f in futures.into_iter() {
        // sleep between each request
        let res = f.await?;
        if let Err(e) = res {
            error!("error: {}", e);
            // retry
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalDataItem {
    pub code: String,
    pub message: String,
    pub timestamp: Value,
    pub datas: Vec<Vec<Value>>,
}

impl FinalDataItem {
    pub fn to_csv_file(&self, file_name: &str) -> Result<()> {
        let mut wtr = csv::Writer::from_path(file_name)?;
        for data in self.datas.iter() {
            let mut record = csv::StringRecord::new();
            for value in data.iter() {
                // replace " to empty
                let value = &serde_json::to_string(value)?.replace("\"", "");
                record.push_field(value);
                // record.push_field();
            }
            wtr.write_record(&record)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

async fn fetch_income_data(
    client: &ClientWithMiddleware,
    stock_code: &str,
) -> Result<FinalDataItem> {
    let url = format!("https://stock.zsxg.cn/api/v2/quarter/depthData?code={}&type=income&style=1&periods=0331%2C0630%2C0930%2C1231", stock_code);
    let resp = client
    .get(url)
    .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36").header("Content-Type", "application/json;charset=UTF-8")
    .send().await?;
    let status_res = resp.error_for_status();
    // if let Err(e) = status_res {
    //     // sleep 1s and retry
    //     tokio::time::sleep(Duration::from_millis(1000)).await;
    //     return fetch_income_data(client, stock_code).await;
    // }
    let rtext = status_res?.text().await?;
    let data: FinalDataItem = serde_json::from_str(&rtext)?;
    Ok(data)
}

async fn fetch_asset_data(
    client: &ClientWithMiddleware,
    stock_code: &str,
) -> Result<FinalDataItem> {
    let url = format!("https://stock.zsxg.cn/api/v2/quarter/depthData?code={}&type=balancesheet&style=1&periods=0331%2C0630%2C0930%2C1231", stock_code);
    let resp = client
    .get(url)
    .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36").header("Content-Type", "application/json;charset=UTF-8")
    .send().await?;
    let rtext = resp.text().await?;
    let data: FinalDataItem = serde_json::from_str(&rtext)?;
    Ok(data)
}

async fn fetch_cash_data(client: &ClientWithMiddleware, stock_code: &str) -> Result<FinalDataItem> {
    let url = format!("https://stock.zsxg.cn/api/v2/quarter/depthData?code={}&type=cashflow&style=1&periods=0331%2C0630%2C0930%2C1231", stock_code);
    let resp = client
    .get(url)
    .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36").header("Content-Type", "application/json;charset=UTF-8")
    .send().await?;
    let status_res = resp.error_for_status();
    // if let Err(e) = status_res {
    //     // sleep 1s and retry
    //     tokio::time::sleep(Duration::from_millis(1000)).await;
    //     return fetch_cash_data(client, stock_code).await;
    // }
    let rtext = status_res?.text().await?;
    let data: FinalDataItem = serde_json::from_str(&rtext)?;
    Ok(data)
}

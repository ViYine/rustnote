use anyhow::Result;
use reqwest::Client;
use tracing::info;

use std::{
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

use lazy_static::lazy_static;
lazy_static! {
    static ref COLUMNS: [&'static str; 15] = [
        "timestamp",      // 时间戳
        "volume",         // 成交量
        "open",           // 开盘价
        "high",           // 最高价
        "low",            // 最低价
        "close",          // 收盘价
        "chg",            // 涨跌额
        "percent",        // 涨跌幅
        "turnoverrate",   // 换手率
        "amount",         // 成交额
        "pe",             // 市盈率
        "pb",             // 市净率
        "ps",             // 市销率
        "pcf",            // 市现率
        "market_capital", // 总市值
    ];
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KlineData {
    pub data: Data,
    pub error_code: i64,
    pub error_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub symbol: String,
    pub column: Vec<String>,
    pub item: Vec<Vec<Option<f64>>>,
}

impl KlineData {
    pub fn to_csv_file(&self) -> Result<()> {
        let file_name = format!("kline_out/{}.csv", self.data.symbol);
        let csv_file = File::create(file_name)?;
        let mut csv_writer = csv::Writer::from_writer(csv_file);
        csv_writer.write_record(&self.data.column)?;
        for row in self.data.item.iter() {
            let mut row_str = Vec::new();
            for item in row.iter() {
                // 第一个元素是时间戳，需要转换成日期
                if row_str.len() == 0 {
                    let time = SystemTime::UNIX_EPOCH
                        + std::time::Duration::from_millis(item.unwrap() as u64);
                    let datetime = chrono::DateTime::<chrono::Local>::from(time);
                    row_str.push(datetime.format("%Y-%m-%d").to_string());
                } else {
                    // 其他元素直接转换成字符串, None 直接转换成空字符串
                    if let Some(item) = item {
                        row_str.push(item.to_string());
                    } else {
                        row_str.push("".to_string());
                    }
                }
            }
            // println!("{:?}", row_str);
            csv_writer.write_record(&row_str)?;
        }
        csv_writer.flush()?;
        Ok(())
    }
}

fn get_cur_time() -> Result<String> {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)?;
    Ok(format!("{}", since_the_epoch.as_millis()))
}

#[tokio::main]
async fn main() -> Result<()> {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::fmt::init();
    let cookie_str = include_str!("../fixtures/xuqiu.cookie");
    let sh_codes = include_str!("../fixtures/SH.code")
        .lines()
        .map(|s| format!("SH{}", s.split('.').next().unwrap()))
        .collect::<Vec<String>>();
    let sz_codes = include_str!("../fixtures/SZ.code")
        .lines()
        .map(|s| format!("SZ{}", s.split('.').next().unwrap()))
        .collect::<Vec<String>>();

    // merge sh and sz codes
    let mut codes = sh_codes;
    codes.extend(sz_codes);
    info!(
        "start download all codes, total: {}, cur_time:{}",
        codes.len(),
        get_cur_time()?
    );

    download_parallel(codes, cookie_str).await?;

    // wait all tasks finish
    // calculate time cost
    info!("time cost: {}", get_cur_time()?);
    // tokio::spawn(async move {
    // for code in codes {
    //     let client = client.clone();
    //     // tokio::spawn(async move {
    //     let _ = get_stock_data_to_csv(&client, &code, cookie_str).await;
    //     // });
    // }
    // // });

    Ok(())
}

async fn download_parallel(codes: Vec<String>, cookie: &'static str) -> Result<()> {
    // all codes download use function: get_stock_data_to_csv
    // use tokio::spawn to download codes in parallel
    let client = Client::new();
    let futures: Vec<_> = codes
        .into_iter()
        .map(|code| {
            let client = client.clone();
            tokio::spawn(async move { get_stock_data_to_csv(&client, &code, cookie).await })
        })
        .collect();

    // do these futures in parallel and return them
    for f in futures.into_iter() {
        let _ = f.await?;
    }
    Ok(())
}
async fn get_stock_data_to_csv(client: &Client, symbol: &str, cookie_str: &str) -> Result<()> {
    info!("start download {}", symbol);
    let cur_time = get_cur_time()?;

    let mut api_url = url::Url::parse("https://stock.xueqiu.com/v5/stock/chart/kline.json?&period=day&type=before&indicator=kline,pe,pb,ps,pcf,market_capital,agt,ggt,balance")?;

    {
        let mut query_pairs = api_url.query_pairs_mut();
        query_pairs.append_pair("symbol", symbol);
        query_pairs.append_pair("begin", &cur_time);
        query_pairs.append_pair("count", "-3000");
    }

    let res_str = client
        .get(api_url.clone())
        // add header for http header
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .header("cookie", cookie_str)
        .send()
        .await? // will cover connection error
        .error_for_status()? // will cover http error
        .text()
        .await?;

    let k_data = serde_json::from_str::<KlineData>(&res_str)?;
    k_data.to_csv_file()?;
    Ok(())
}

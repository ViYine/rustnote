use color_eyre::eyre::Result;
use qurant_trade::back_test::BackTestMarketInfo;
use tracing::info;
fn main() -> Result<()> {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::fmt::init();
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
    info!("all read stock codes len: {:?}", codes.len());

    let back_trace_market_infos = BackTestMarketInfo::new(
        "2020-01-01",
        "2020-12-31",
        "./kline_out/",
        &codes.as_slice()[1..10],
    )?;
    info!("back_trace_market_infos: {:?}", back_trace_market_infos);

    Ok(())
}

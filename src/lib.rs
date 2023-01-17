pub mod back_test;
pub mod trade_strategy;
use chrono::NaiveDate;
use std::default;

pub use back_test::{MarketInfo, PositionInfo};
pub use trade_strategy::TargetTrade;

#[warn(unreachable_patterns)]
/// 交易记录
#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub stock_code: String,
    pub trade_type: TradeType,
    pub trade_price: f64,
    pub trade_amount: f64,
    pub trade_date: NaiveDate,
    // 交易结果：成功，失败，未知
    pub trade_result: TradeResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeResult {
    Unknow,
    Success,
    Fail,
}

impl default::Default for TradeResult {
    fn default() -> Self {
        TradeResult::Unknow
    }
}

#[derive(Debug, Clone)]
pub enum TradeType {
    Buy(TradeStrategy),
    Sell(TradeStrategy),
}

#[derive(Debug, Clone)]
pub enum TradeStrategy {
    // 交易策略
    // 设定目标价格，当价格高于目标价格时卖出
    TargetPrice(f64),
    // 指定日期开盘进行操作
    OpenDate,
    // 指定日期收盘进行操作
    CloseDate,
}

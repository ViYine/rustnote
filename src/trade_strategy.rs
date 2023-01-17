use chrono::NaiveDate;
use color_eyre::eyre::{eyre, Result};

use crate::{MarketInfo, TradeRecord, TradeResult, TradeStrategy, TradeType};

// 1. 回测交易的基本信息, 由交易策略产生
#[derive(Debug, Clone)]
pub struct TargetTrade {
    pub stock_code: String,
    // 交易的价格, 如果没有指定价格则以date的策略进行交易
    pub trade_type: TradeType,
    // 交易的数量
    pub target_amount: f64,
    // 交易的日期
    pub trade_date: Option<NaiveDate>,
}

impl TargetTrade {
    pub fn new(
        stock_code: String,
        trade_type: TradeType,
        target_amount: f64,
        trade_date_str: &str,
    ) -> Result<Self> {
        if target_amount <= 0.0 {
            return Err(eyre!("target_amount must be greater than 0"));
        }
        if trade_date_str.is_empty() {
            return Ok(Self {
                stock_code,
                trade_type,
                target_amount,
                trade_date: None,
            });
        }
        let trade_date = NaiveDate::parse_from_str(trade_date_str, "%Y-%m-%d")?;
        Ok(Self {
            stock_code,
            trade_type,
            target_amount,
            trade_date: Some(trade_date),
        })
    }

    pub fn execute(
        &self,
        market_info: &MarketInfo,
        cash_amt: f64, // how much cash we have
    ) -> Result<Option<TradeRecord>> {
        if market_info.stock_code != self.stock_code {
            return Ok(None);
        }

        let market_trade_date =
            NaiveDate::parse_from_str(&market_info.detail_info.timestamp, "%Y-%m-%d")?;
        let mut trade_price = market_info.detail_info.close; // default trade price

        if let Some(trade_date) = self.trade_date {
            // 如果指定了交易日期，则只有指定日期才去判断价格是否满足
            if trade_date != market_trade_date {
                return Ok(None);
            }
        };

        match self.trade_type {
            TradeType::Buy(TradeStrategy::TargetPrice(target_price)) => {
                // 指定目标价格买入
                // 如果当天的最低-最高价的范围包含目标价格，则价格信息上可以执行买入操作
                // 如果买入的数量大于现金，则买入失败
                if market_info.detail_info.high >= target_price {
                    trade_price = target_price;
                }
            }

            TradeType::Buy(TradeStrategy::OpenDate) => {
                // 指定日期开盘买入
                if self.trade_date.is_none() {
                    return Err(eyre!("OpenDate strategy must specify trade_date"));
                }
                trade_price = market_info.detail_info.open;
            }

            TradeType::Buy(TradeStrategy::CloseDate) => {
                // 指定日期收盘买入
                if self.trade_date.is_none() {
                    return Err(eyre!("CloseDate strategy must specify trade_date"));
                }
                trade_price = market_info.detail_info.close;
            }

            TradeType::Sell(TradeStrategy::TargetPrice(target_price)) => {
                // 指定目标价格卖出
                // 如果当天的最低-最高价的范围包含目标价格，则价格信息上可以执行卖出操作
                // 如果卖出的数量大于持仓，则卖出失败
                if market_info.detail_info.high >= target_price {
                    trade_price = target_price;
                }
            }

            TradeType::Sell(TradeStrategy::OpenDate) => {
                // 指定日期开盘卖出
                if self.trade_date.is_none() {
                    return Err(eyre!("OpenDate strategy must specify trade_date"));
                }
                trade_price = market_info.detail_info.open;
            }

            TradeType::Sell(TradeStrategy::CloseDate) => {
                // 指定日期收盘卖出
                if self.trade_date.is_none() {
                    return Err(eyre!("CloseDate strategy must specify trade_date"));
                }
                trade_price = market_info.detail_info.close;
            }
        };

        // 判断是否可以交易
        let need_cash = trade_price * self.target_amount;
        let mut trade_result = TradeResult::Success;
        if need_cash > cash_amt {
            trade_result = TradeResult::Fail;
        }
        Ok(Some(TradeRecord {
            stock_code: self.stock_code.clone(),
            trade_type: self.trade_type.clone(),
            trade_price,
            trade_amount: self.target_amount,
            trade_date: market_trade_date,
            trade_result,
        }))
    }
}

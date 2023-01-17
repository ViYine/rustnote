use std::collections::HashMap;

use crate::{TargetTrade, TradeRecord, TradeResult, TradeType};
use chrono::NaiveDate;
use color_eyre::eyre::{eyre, Result};
use serde::Deserialize;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct BackTestMarketInfo {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub market_info_map: HashMap<NaiveDate, Vec<MarketInfo>>,
}

/// 市场信息，会按照日期进行回放
#[derive(Debug, Clone, Default)]
pub struct MarketInfo {
    pub stock_code: String,
    pub detail_info: StockDetailInfo,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StockDetailInfo {
    pub timestamp: String,
    pub volume: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub chg: f64,
    pub percent: f64,
    pub turnoverrate: f64,
    pub amount: Option<f64>,
    pub volume_post: Option<f64>,
    pub amount_post: Option<f64>,
    pub pe: Option<f64>,
    pub pb: Option<f64>,
    pub ps: Option<f64>,
    pub pcf: Option<f64>,
    pub market_capital: Option<f64>,
    pub balance: Option<f64>,
    pub hold_volume_cn: Option<f64>,
    pub hold_ratio_cn: Option<f64>,
    pub net_volume_cn: Option<f64>,
    pub hold_volume_hk: Option<f64>,
    pub hold_ratio_hk: Option<f64>,
    pub net_volume_hk: Option<f64>,
}

/// 持仓明细, 当前还没卖出的stock
#[derive(Debug, Clone)]
pub struct PositionDetail {
    pub stock_code: String,
    // 持有的总数量
    pub stock_amount: f64,
    // 持有的总成本
    pub stock_total_cost: f64,
    // 持有的总市值
    pub stock_total_value: f64,
    // 持有的总盈亏
    pub stock_total_profit: f64,
    // 持有的总盈亏率
    pub stock_total_profit_rate: f64,
    // 持有的最大盈利
    pub stock_max_profit: f64,
    // 持有的最大亏损
    pub stock_max_loss: f64,
    // 持有的最大盈利率
    pub stock_max_profit_rate: f64,
    // 持有的最大亏损率
    pub stock_max_loss_rate: f64,
    // 对应的交易记录
    pub trade_records: Vec<TradeRecord>,
    // 最新市场信息
    pub last_market_info: MarketInfo,
    // 当前持仓 执行操作所需要的资金，默认为0，如果为负数，表示卖出所得的现金，如果为正数，表示买入需要的现金
    pub need_cash: f64,
}

impl PositionDetail {
    pub fn add_trade_record(&mut self, trade_record: &TradeRecord) -> Result<()> {
        if self.stock_code != trade_record.stock_code {
            // 不需要处理，直接返回
            return Err(eyre!("stock code not match"));
        }
        self.trade_records.push(trade_record.clone());
        // 如果交易结果为success 才更新持仓信息
        if trade_record.trade_result != TradeResult::Success {
            return Ok(());
        }

        let trade_cash = trade_record.trade_price * trade_record.trade_amount;

        // 判断交易类型，进行买入或者卖出的之后的持仓信息更新
        match trade_record.trade_type {
            TradeType::Buy(_) => {
                self.stock_amount += trade_record.trade_amount;
                self.stock_total_cost += trade_cash;
                self.need_cash += trade_cash;
            }
            TradeType::Sell(_) => {
                self.stock_amount -= trade_record.trade_amount;
                self.stock_total_cost -= trade_cash;
                self.need_cash -= trade_cash;
            }
        }
        // 更新收益信息
        self.update_profit_info()?;

        Ok(())
    }

    fn update_profit_info(&mut self) -> Result<()> {
        // 计算当前持仓的总市值：持仓数量 * 最新价格 - 需要的现金（为负数表示卖出获得的现金）
        self.stock_total_value =
            self.stock_amount * self.last_market_info.detail_info.close - self.need_cash;
        // 计算当前持仓的总盈亏
        self.stock_total_profit = self.stock_total_value - self.stock_total_cost;
        // 计算当前持仓的总盈亏率
        self.stock_total_profit_rate = self.stock_total_profit / self.stock_total_cost;
        // 计算当前持仓的最大盈利
        if self.stock_total_profit > self.stock_max_profit {
            self.stock_max_profit = self.stock_total_profit;
            self.stock_max_profit_rate = self.stock_max_profit / self.stock_total_cost;
        }
        // 计算当前持仓的最大亏损
        if self.stock_total_profit < self.stock_max_loss {
            self.stock_max_loss = self.stock_total_profit;
            self.stock_max_loss_rate = self.stock_max_loss / self.stock_total_cost;
        }
        Ok(())
    }
}

/// 持仓信息
#[derive(Debug, Clone, Default)]
pub struct PositionInfo {
    // 以stock_code为key
    pub position_record_map: HashMap<String, PositionDetail>,
    pub current_cost: f64,
    // 当前持仓的总市值
    pub current_value: f64,
    // 当前持仓的盈亏
    pub current_profit: f64,
    // 当前持仓的盈亏率
    pub current_profit_rate: f64,
    // 当前持仓的最大盈利
    pub max_profit: f64,
    // 当前持仓的最大亏损
    pub max_loss: f64,
    // 当前持仓的最大盈利率
    pub max_profit_rate: f64,
    // 当前持仓的最大亏损率
    pub max_loss_rate: f64,
    // 当前持仓的最大盈利日期
    pub max_profit_date: NaiveDate,
    // 当前持仓的最大亏损日期
    pub max_loss_date: NaiveDate,
    // 最新的日期
    pub last_date: NaiveDate,
}

impl PositionInfo {
    pub fn new() -> Self {
        Self {
            position_record_map: HashMap::new(),
            current_cost: 0.0,
            current_value: 0.0,
            current_profit: 0.0,
            current_profit_rate: 0.0,
            max_profit: 0.0,
            max_loss: 0.0,
            max_profit_rate: 0.0,
            max_loss_rate: 0.0,
            max_profit_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            max_loss_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            last_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
        }
    }

    pub fn update_by_market_info(&mut self, market_info: &MarketInfo) -> Result<()> {
        let stock_code = &market_info.stock_code;
        if self.position_record_map.contains_key(stock_code) {
            // 更新单个stock明细
            let mut position_detail = self.position_record_map.get_mut(stock_code).unwrap();
            position_detail.last_market_info = market_info.clone();
            position_detail.stock_total_value = position_detail.stock_amount
                * market_info.detail_info.close
                - position_detail.need_cash;
            position_detail.stock_total_profit =
                position_detail.stock_total_value - position_detail.stock_total_cost;
            position_detail.stock_total_profit_rate =
                position_detail.stock_total_profit / position_detail.stock_total_cost;
            if position_detail.stock_total_profit > position_detail.stock_max_profit {
                position_detail.stock_max_profit = position_detail.stock_total_profit;
                position_detail.stock_max_profit_rate = position_detail.stock_total_profit_rate;
            }
            if position_detail.stock_total_profit < position_detail.stock_max_loss {
                position_detail.stock_max_loss = position_detail.stock_total_profit;
                position_detail.stock_max_loss_rate = position_detail.stock_total_profit_rate;
            }
        }

        // 更新持仓的汇总信息
        self.current_cost = self
            .position_record_map
            .values()
            .fold(0.0, |acc, x| acc + x.stock_total_cost);
        self.current_value = self
            .position_record_map
            .values()
            .fold(0.0, |acc, x| acc + x.stock_total_value);
        self.current_profit = self.current_value - self.current_cost;
        self.current_profit_rate = self.current_profit / self.current_cost;
        let cur_date = NaiveDate::parse_from_str(&market_info.detail_info.timestamp, "%Y-%m-%d")?;
        if self.current_profit > self.max_profit {
            self.max_profit = self.current_profit;
            self.max_profit_rate = self.current_profit_rate;
            self.max_profit_date = cur_date;
        }
        if self.current_profit < self.max_loss {
            self.max_loss = self.current_profit;
            self.max_loss_rate = self.current_profit_rate;
            self.max_loss_date = cur_date;
        }
        self.last_date = cur_date;

        Ok(())
    }

    fn add_trade_record(&mut self, trade_record: TradeRecord) -> Result<()> {
        let stock_code = &trade_record.stock_code;
        if self.position_record_map.contains_key(stock_code) {
            let position_detail = self.position_record_map.get_mut(stock_code).unwrap();
            position_detail.add_trade_record(&trade_record)?;
        }
        Ok(())
    }
}

/// 交易回测的初始信息
#[derive(Debug, Clone, Default)]
pub struct TradeBackTestInfo {
    // 初始资金
    pub init_cash: f64,
    // 开始日期
    pub start_date: NaiveDate,
    // 结束日期
    pub end_date: NaiveDate,
    // 目标收益
    pub target_profit: f64,
    // 目标收益率
    pub target_profit_rate: f64,
}

impl TradeBackTestInfo {
    pub fn new(
        init_cash: f64,
        start_date: &str,
        end_date: &str,
        target_profit: f64,
        target_profit_rate: f64,
    ) -> Result<Self> {
        let s_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
        let e_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?;
        Ok(TradeBackTestInfo {
            init_cash,
            start_date: s_date,
            end_date: e_date,
            target_profit,
            target_profit_rate,
        })
    }
}

/// 交易回测的结果
#[derive(Debug, Clone, Default)]
pub struct TradeBackTestResult {
    // 交易回测的初始信息
    pub trade_back_test_info: TradeBackTestInfo,
    // 持仓信息
    pub position_info: PositionInfo,
    // 最终的现金
    pub final_cash: f64,
    // 最终的收益
    pub final_profit: f64,
    // 最终的收益率
    pub final_profit_rate: f64,
    // 年化收益率
    pub annual_profit_rate: f64,
}

impl TradeBackTestResult {
    /// 初始化一个交易回测的info
    pub fn new(
        init_cash: f64,
        start_date: &str,
        end_date: &str,
        target_profit: f64,
        target_profit_rate: f64,
    ) -> Result<Self> {
        let init_info = TradeBackTestInfo::new(
            init_cash,
            start_date,
            end_date,
            target_profit,
            target_profit_rate,
        )?;
        Ok(TradeBackTestResult {
            trade_back_test_info: init_info,
            position_info: PositionInfo::default(),
            final_cash: init_cash,
            final_profit: 0.0,
            final_profit_rate: 0.0,
            annual_profit_rate: 0.0,
        })
    }

    /// update the position info by market info
    pub fn update(
        &mut self,
        market_info: &MarketInfo,
        trade_strategy: Option<&TargetTrade>,
    ) -> Result<()> {
        // 交易策略执行，添加交易记录，以及更新持仓信息
        if let Some(trade_strategy) = trade_strategy {
            let trade_record = trade_strategy.execute(market_info, self.final_cash)?;
            if let Some(trade_record) = trade_record {
                self.position_info.add_trade_record(trade_record)?;
            }
        }
        // 只用市场信息，更新持仓信息
        self.position_info.update_by_market_info(market_info)?;

        // todo 更新最终的现金

        Ok(())
    }
}

impl BackTestMarketInfo {
    pub fn new(
        s_date: &str,
        e_date: &str,
        csv_data_path_pre: &str,
        stock_codes: &[String],
    ) -> Result<Self> {
        let start_date = NaiveDate::parse_from_str(s_date, "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str(e_date, "%Y-%m-%d")?;
        let mut market_info_map: HashMap<NaiveDate, Vec<MarketInfo>> = HashMap::new();

        for st_code in stock_codes {
            let csv_data_path = format!("{}{}.csv", csv_data_path_pre, st_code);
            info!("to read csv_data_path: {}", csv_data_path);
            let csv_reader = csv::Reader::from_path(csv_data_path);
            if let Err(e) = csv_reader {
                error!(
                    "read csv_data_path: {}{}.csv error: {:?}",
                    csv_data_path_pre, st_code, e
                );
                continue;
            }
            for result in csv_reader.unwrap().deserialize() {
                let record: StockDetailInfo = result?;
                let date = NaiveDate::parse_from_str(&record.timestamp, "%Y-%m-%d")?;
                if date < start_date || date > end_date {
                    continue;
                }
                let market_info = MarketInfo {
                    stock_code: st_code.clone(),
                    detail_info: record,
                };

                let date_data = market_info_map.get_mut(&date);
                if let Some(date_data) = date_data {
                    date_data.push(market_info);
                } else {
                    market_info_map.insert(date, vec![market_info]);
                }
            }
        }
        Ok(BackTestMarketInfo {
            market_info_map,
            start_date,
            end_date,
        })
    }
}

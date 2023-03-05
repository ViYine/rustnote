
use color_eyre::eyre::{Result, eyre};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[tokio::main]
async fn main() -> Result<()> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;
    let stock_code = "300363.SZ";
    let income_data = fetch_income_data(&client, stock_code).await?;
    income_data.to_csv_file(&format!("finial_data/{}_income.csv", stock_code))?;
    let asset_data = fetch_asset_data(&client, stock_code).await?;
    asset_data.to_csv_file(&format!("finial_data/{}_asset.csv", stock_code))?;
    let cash_data = fetch_cash_data(&client, stock_code).await?;
    cash_data.to_csv_file(&format!("finial_data/{}_cash.csv", stock_code))?;
    let mut base_financial_index = BaseFinancialIndicator::new(stock_code);
    base_financial_index.from_fininal_value((&income_data.datas[0], &income_data.datas[1]))?;
    println!("{}", serde_json::to_string(&asset_data.datas[0])?);
    base_financial_index.from_fininal_value((&asset_data.datas[0], &asset_data.datas[1]))?;
    base_financial_index.from_fininal_value((&cash_data.datas[0], &cash_data.datas[1]))?;
    println!("base_financial_index: {:?}", base_financial_index);

    println!("{:?}", income_data.datas[0].len());
    println!("{}", serde_json::to_string(&income_data.datas[0])?);
    println!("{:?}", asset_data.datas[0].len());
    println!("{}", serde_json::to_string(&asset_data.datas[0])?);
    println!("{:?}", cash_data.datas[0].len());
    println!("cash_data {:?}", &cash_data.datas[0]);
    // let index_n = income_data.datas[0];
    let index_n = &cash_data.datas[0].iter().map(|x| 
        x.as_str().unwrap().trim().replace("*", "")).collect::<Vec<String>>();

    println!("index_n {:?}", index_n);
    let index_v: Vec<f64> = cash_data.datas[1].iter().map(|x| x.as_f64().unwrap_or_default()).collect();

    println!("index_v {:?}", index_v);

    let cash_index = &cash_data.datas[0];
    let asset_index = &asset_data.datas[0];
    let income_index = &income_data.datas[0];
    let cash_table_sql = format!("create table cash_index_data (st_code varchar(128) not null default '',\n{}\n primary key(st_code,报告期)\n )engine=innodb default charset=utf8;", cash_index.iter().map(|x| {
        let s = x.to_string();
        if s.contains("报告期") || s.contains("公告日期") {
            format!("{} date",x.as_str().unwrap().replace('*', " ").replace("：", ""))
        } else {
            format!("{} DECIMAL(19,4) default null",x.as_str().unwrap().replace('*', " ").replace("：", ""))
        }
    } ).collect::<Vec<String>>().join(",\n"));
    let asset_table_sql = format!("create table asset_index_data (st_code varchar(128) not null default '',\n{}\n primary key(st_code,报告期)\n )engine=innodb default charset=utf8;", asset_index.iter().map(|x| {
        let s = x.to_string();
        if s.contains("报告期") || s.contains("公告日期") {
            format!("{} date", x.as_str().unwrap().replace('*', " ").replace("：", " "))
        } else {
            format!("{} DECIMAL(19,4) default null", x.as_str().unwrap().replace('*', " ").replace("：", ""))
        }
    } ).collect::<Vec<String>>().join(",\n"));

    let income_table_sql = format!("create table income_index_data (st_code varchar(128) not null default '',\n{}\n primary key(st_code,报告期)\n )engine=innodb default charset=utf8;", income_index.iter().map(|x| {
        let s = x.to_string();
        if s.contains("报告期") || s.contains("公告日期") {
            format!("{} date", x.as_str().unwrap().replace('*', " ").replace("：", ""))
        } else {
            format!("{} DECIMAL(19,4) default null", x.as_str().unwrap().replace('*', " ").replace("：", ""))
        }
    } ).collect::<Vec<String>>().join(",\n"));

    // println!("{}", cash_table_sql);
    // println!("{}", asset_table_sql);
    // println!("{}", income_table_sql);

    Ok(())

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profitability {
    // 盈利能力指标
    pub st_code: String,
    pub report_date: String,
    pub cur_date: String,
    pub roe: f64,
    pub roa: f64,
    // 毛利率
    pub gross_profit_margin: f64,
    // 净利率
    pub net_profit_margin: f64,
    // 主营利润率
    pub profite_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Solvency {
    // 清债能力指标
    pub st_code: String,
    pub report_date: String,
    pub cur_date: String,
    // 资产负债率
    pub debt_rate: f64,
    // 权益乘数
    pub debt_equity_ratio: f64,
    // 利息保障倍数
    pub interest_coverage_ratio: f64,
    // 自由现金流
    pub free_cash_flow: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub st_code: String,
    pub report_date: String,
    pub cur_date: String,
    // 存货周转率
    pub inventory_turnover: f64,
    // 应收账款周转率
    pub accounts_receivable_turnover: f64,
    // 总资产周转率
    pub total_assets_turnover: f64,
    // 固定资产周转率
    pub fixed_assets_turnover: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Growth {
    pub st_code: String,
    pub report_date: String,
    pub cur_date: String,
    // 营业收入增长率
    pub revenue_growth_rate: f64,
    // 净利润增长率
    pub net_profit_growth_rate: f64,
    // 总资产增长率
    pub total_assets_growth_rate: f64,
    // 净资产增长率
    pub net_assets_growth_rate: f64,
    // 存货增长率
    pub inventory_growth_rate: f64,
    // 固定资产增长率
    pub fixed_assets_growth_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashFlowIndex {
    pub st_code: String,
    pub report_date: String,
    pub cur_date: String,
    // 现金流量比率 = 经营活动产生的现金流量净额/营业收入
    pub cash_flow_ratio: f64,
    // 债务保障比率 = 经营活动产生的现金流量净额/（流动负债合计+长期借款）
    pub debt_protection_ratio: f64,
    // 现金流量充裕度 = 自由现金流/经营活动产生的现金流量净额
    pub cash_flow_surplus: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketIndex {
    pub st_code: String,
    pub report_date: String,
    pub cur_date: String,
    // 基本每股收益
    pub basic_eps: f64,
    // 市盈率
    pub pe: f64,
    // 市净率
    pub pb: f64,
    // 市销率
    pub ps: f64,
    // 市现率
    pub pc: f64,
    // 市现率
    pub pcf: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseFinancialIndicator {
    // 基本财务指标信息
    pub st_code: String,
    pub report_date: String,
    // 流动资产总计
    pub total_current_assets: f64,
    // 货币资金
    pub money_funds: f64,
    // 应收账款
    pub accounts_receivable: f64,
    // 交易性金融资产  
    pub trading_financial_assets: f64, 
    // 存货
    pub inventory: f64,
    // 其他流动资产
    pub other_current_assets: f64,
    // 固定资产
    pub fixed_assets: f64,
    // 流动负债合计
    pub total_current_liabilities: f64,
    // 长期借款
    pub long_term_loan: f64,
    // 负债合计
    pub total_liabilities: f64,
    // 所有者权益合计
    pub total_owner_equities: f64,
    // 少数股东权益
    pub minority_interests: f64,
    // 资产总计
    pub total_assets: f64,
    // 营业收入
    pub operating_revenue: f64,
    // 营业成本
    pub operating_costs: f64,
    // 财务费用
    pub financial_expenses: f64,
    // 管理费用
    pub management_expenses: f64,
    // 销售费用
    pub sales_expenses: f64,
    // 营业利润
    pub operating_profit: f64,
    // 利润总额
    pub total_profit: f64,
    // 净利润
    pub net_profit: f64,
    // 经营活动产生的现金流量净额
    pub net_cash_flow_from_operating_activities: f64,
    // 构建固定资产、无形资产和其他长期资产支付的现金
    pub cash_paid_for_intangible_assets: f64,
    // 处置固定资产、无形资产和其他长期资产收回的现金净额
    pub net_cash_from_disposal_of_intangible_assets: f64,
    // 基本每股收益
    pub basic_eps: f64,
}

impl BaseFinancialIndicator {
    pub fn new(stock_code: &str) -> Self {
        Self {
            st_code: stock_code.to_string(),
            ..Default::default()
        }
    }
    pub fn from_fininal_value(&mut self, item: (&[Value], &[Value])) -> Result<()> {
        let (index_name, index_value) = item;
        // index_name is arr of index name, index_value is arr of index value
        let index_name_list = index_name.iter().map(|x| 
            x.as_str().unwrap().trim().replace("*", "")
        ).collect::<Vec<String>>();
        let index_value_list:Vec<f64> = index_value.iter().map(|x| 
            x.as_f64().unwrap_or_default()
        ).collect();
        // 报告期
        // let index_i = index_name_list[..].index("报告期".to_string()).unwrap();
        self.report_date = index_value[0].as_str().unwrap_or_else(|| "").to_string();
        // 流动资产总计
        // find index of "流动资产总计" from index_name_list, and get the value from index_value_list
        let index_i = index_name_list[..].iter().position(|x| x == "流动资产总计");
        if let Some(index_i) = index_i {
            self.total_current_assets = index_value_list[index_i];
        }
        // 货币资金
        let index_i = index_name_list[..].iter().position(|x| x == "货币资金");
        if let Some(index_i) = index_i {
            self.money_funds = index_value_list[index_i];
        }
        // 应收账款
        let index_i = index_name_list[..].iter().position(|x| x == "应收账款");
        if let Some(index_i) = index_i {
            self.accounts_receivable = index_value_list[index_i];
        }
        // 交易性金融资产
        let index_i = index_name_list[..].iter().position(|x| x == "交易性金融资产");
        if let Some(index_i) = index_i {
            self.trading_financial_assets = index_value_list[index_i];
        }
        // 存货
        let index_i = index_name_list[..].iter().position(|x| x == "存货");
        if let Some(index_i) = index_i {
            self.inventory = index_value_list[index_i];
        }
        // 其他流动资产
        let index_i = index_name_list[..].iter().position(|x| x == "其他流动资产");
        if let Some(index_i) = index_i {
            self.other_current_assets = index_value_list[index_i];
        }
        // 固定资产
        let index_i = index_name_list[..].iter().position(|x| x == "固定资产");
        if let Some(index_i) = index_i {
            self.fixed_assets = index_value_list[index_i];
        }
        // 流动负债合计
        let index_i = index_name_list[..].iter().position(|x| x == "流动负债合计");
        if let Some(index_i) = index_i {
            self.total_current_liabilities = index_value_list[index_i];
        }
        // 长期借款
        let index_i = index_name_list[..].iter().position(|x| x == "长期借款");
        if let Some(index_i) = index_i {
            self.long_term_loan = index_value_list[index_i];
        }
        // 负债合计
        let index_i = index_name_list[..].iter().position(|x| x == "负债合计");
        if let Some(index_i) = index_i {
            self.total_liabilities = index_value_list[index_i];
        }
        // 所有者权益合计
        let index_i = index_name_list[..].iter().position(|x| x == "所有者权益合计");
        if let Some(index_i) = index_i {
            self.total_owner_equities = index_value_list[index_i];
        }
        // 少数股东权益
        let index_i = index_name_list[..].iter().position(|x| x == "少数股东权益");
        if let Some(index_i) = index_i {
            self.minority_interests = index_value_list[index_i];
        }
        // 资产总计
        let index_i = index_name_list[..].iter().position(|x| x == "资产总计");
        if let Some(index_i) = index_i {
            self.total_assets = index_value_list[index_i];
        }
        // 营业收入
        let index_i = index_name_list[..].iter().position(|x| x == "营业收入");
        if let Some(index_i) = index_i {
            self.operating_revenue = index_value_list[index_i];
        }
        // 营业成本
        let index_i = index_name_list[..].iter().position(|x| x == "营业成本");
        if let Some(index_i) = index_i {
            self.operating_costs = index_value_list[index_i];
        }
        // 财务费用
        let index_i = index_name_list[..].iter().position(|x| x == "财务费用");
        if let Some(index_i) = index_i {
            self.financial_expenses = index_value_list[index_i];
        }
        // 管理费用
        let index_i = index_name_list[..].iter().position(|x| x == "管理费用");
        if let Some(index_i) = index_i {
            self.management_expenses = index_value_list[index_i];
        }
        // 销售费用
        let index_i = index_name_list[..].iter().position(|x| x == "销售费用");
        if let Some(index_i) = index_i {
            self.sales_expenses = index_value_list[index_i];
        }
        // 营业利润
        let index_i = index_name_list[..].iter().position(|x| x == "营业利润");
        if let Some(index_i) = index_i {
            self.operating_profit = index_value_list[index_i];
        }
        // 利润总额
        let index_i = index_name_list[..].iter().position(|x| x == "利润总额");
        if let Some(index_i) = index_i {
            self.total_profit = index_value_list[index_i];
        }
        // 净利润
        let index_i = index_name_list[..].iter().position(|x| x == "净利润");
        if let Some(index_i) = index_i {
            if self.net_profit == 0.0 {
                self.net_profit = index_value_list[index_i];
            }
        }
        // 经营活动产生的现金流量净额
        let index_i = index_name_list[..].iter().position(|x| x == "经营活动产生的现金流量净额");
        if let Some(index_i) = index_i {
            if self.net_cash_flow_from_operating_activities == 0.0 {
                self.net_cash_flow_from_operating_activities = index_value_list[index_i];
            }
        }
        // 构建固定资产、无形资产和其他长期资产支付的现金
        let index_i = index_name_list[..].iter().position(|x| x == "构建固定资产、无形资产和其他长期资产支付的现金");
        if let Some(index_i) = index_i {
            if self.cash_paid_for_intangible_assets == 0.0 {
                self.cash_paid_for_intangible_assets = index_value_list[index_i];
            }
        }
        // 处置固定资产、无形资产和其他长期资产收回的现金净额
        let index_i = index_name_list[..].iter().position(|x| x == "处置固定资产、无形资产和其他长期资产收回的现金净额");
        if let Some(index_i) = index_i {
            if self.net_cash_from_disposal_of_intangible_assets == 0.0 {
                self.net_cash_from_disposal_of_intangible_assets = index_value_list[index_i];
            }
        }
        // 基本每股收益
        let index_i = index_name_list[..].iter().position(|x| x == "基本每股收益");
        if let Some(index_i) = index_i {
            self.basic_eps = index_value_list[index_i];
        }
        // 每股净资产
        Ok(())
    }
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
    

async fn fetch_income_data(client: &reqwest::Client, stock_code: &str) -> Result<FinalDataItem> {
    let url = format!("https://stock.zsxg.cn/api/v2/quarter/depthData?code={}&type=income&style=1&periods=0331%2C0630%2C0930%2C1231", stock_code);
    let resp = client
    .get(url)
    .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36").header("Content-Type", "application/json;charset=UTF-8")
    .send().await?;
    let data: FinalDataItem = serde_json::from_str(&resp.text().await?)?;
    Ok(data)
}

async fn fetch_asset_data(client: &reqwest::Client, stock_code: &str) -> Result<FinalDataItem> {
    let url = format!("https://stock.zsxg.cn/api/v2/quarter/depthData?code={}&type=balancesheet&style=1&periods=0331%2C0630%2C0930%2C1231", stock_code);
    let resp = client
    .get(url)
    .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36").header("Content-Type", "application/json;charset=UTF-8")
    .send().await?;
    let data: FinalDataItem = serde_json::from_str(&resp.text().await?)?;
    Ok(data)
}


async fn fetch_cash_data(client: &reqwest::Client, stock_code: &str) -> Result<FinalDataItem> {
    let url = format!("https://stock.zsxg.cn/api/v2/quarter/depthData?code={}&type=cashflow&style=1&periods=0331%2C0630%2C0930%2C1231", stock_code);
    let resp = client
    .get(url)
    .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36").header("Content-Type", "application/json;charset=UTF-8")
    .send().await?;
    let data: FinalDataItem = serde_json::from_str(&resp.text().await?)?;
    Ok(data)
}

// 计算基本面数据的trait
trait CalcFromFinalDataItem {
    fn calc(income: &FinalDataItem, asset: &FinalDataItem, cash: &FinalDataItem) -> Self;
}

impl CalcFromFinalDataItem for Profitability {
    fn calc(income: &FinalDataItem, asset: &FinalDataItem, cash: &FinalDataItem) -> Self {
        todo!("implement calc for Profitability");
    }
    
}
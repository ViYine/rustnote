
# api接口 公共参数说明

style=1 表示为原始报表
style=2 表示为按照单季度的报表
periods=单个季度的日期，就是只获取对应季度的报表数据；如果等于多个季度日期就是等于多季度的排列

## 利润表接口api 

curl 'https://stock.zsxg.cn/api/v2/quarter/depthData?code=300363.SZ&type=income&style=2&periods=0331%2C0630%2C0930%2C1231' \
  -H 'Content-Type: application/json;charset=utf-8' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36'


### 盈利能力计算相关指标公式

ROE（净资产收益率）= 净利润/净资产（所有者权益合计）
ROA（总资产收益率）= 净利润/总资产（资产总计）
GrossRate(毛利率) =（营业收入-营业成本）/营业收入
ProfitRate(主营业务利润率) = 业务利润/营业收入
NetProfitRate(净利率) = 净利润/营业收入

### 偿债能力 流动性水平

FluentRate (流动比率) = 流动资产总计/流动负债合计
QuickRate (速动比率) = （流动资产总计-存货-其他流动资产）/流动负债合计
CashRate (现金比率) = （货币资金+交易性金融资产）/流动负债合计

### 清偿能力

DebtRate (资产负债率) = 负债合计/资产总计
DebtEquityRate (权益乘数) = 负债合计/（所有者权益合计-少数股东权益）
InterestCover 利息保障倍数 = EBIT（利润总额+财务费用）/财务费用
FreeCashFlow 自由现金流量 = 经营活动产生的现金流量净额-资本支出（构建固定资产、无形资产和其他长期资产支付的现金-处置固定资产、无形资产和其他长期资产收回的现金净额）

### 营运能力

InventoryTurnoverRate 存货周转率 = 营业成本/平均存货(存货+期初存货)/2
AccountRecvTurnover 应收账款周转率 = 营业收入/平均应收账款(应收账款+期初应收账款)/2
FixAssetTurnover 固定资产周转率 = 营业收入/平均固定资产(固定资产+期初固定资产)/2
TotalAssetTurnover 总资产周转率 = 营业收入/平均总资产(资产总计+期初资产总计)/2

### 成长能力

营业收入增长率 = （本期营业收入-上期营业收入）/上期营业收入
净利润增长率 = （本期净利润-上期净利润）/上期净利润
净资产增长率 = （本期净资产（所有者权益合计）-上期净资产）/上期净资产
总资产增长率 = （本期总资产（资产总计）-上期总资产）/上期总资产
固定资产增长率 = （本期固定资产-上期固定资产）/上期固定资产
存货增长率 = （本期存货-上期存货）/上期存货

### 现金流情况

CashFlowRate 现金流量比率 = 经营活动产生的现金流量净额/营业收入
债务保障倍数 = 经营活动产生的现金流量净额/（流动负债合计+长期借款）
自由现金流与经营活动净现金流比值 = 自由现金流/经营活动产生的现金流量净额

### 市场表现

EPS = 基本每股收益 = 归属母公司普通股东综合收益总额/普通股股数
普通股股数 = 归属母公司普通股东综合收益总额 / 基本每股收益
PE = 市盈率 = 市场价格/每股收益
PB = 市净率 = 市场价格/每股净资产
PS = 市销率 = 市场价格/每股销售额


## 现金流量表 api

curl 'https://stock.zsxg.cn/api/v2/quarter/depthData?code=300363.SZ&type=cashflow&style=1&periods=0331%2C0630%2C0930%2C1231' \
  -H 'Content-Type: application/json;charset=utf-8' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36'


## 资产负债表 api

curl 'https://stock.zsxg.cn/api/v2/quarter/depthData?code=300363.SZ&type=balancesheet&style=1&periods=0331%2C0630%2C0930%2C1231' \
-H 'Content-Type: application/json;charset=utf-8' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36'


## 盘口数据



curl 'https://stock.xueqiu.com/v5/stock/quote.json?symbol=SH600000&extend=detail' \
  -H 'accept: */*' \
  -H 'accept-language: zh-CN,zh;q=0.9' \
  -H 'cookie: s=cy16gh65r8; device_id=007cbfbd9efc7d152f53bad187e693e4; xq_a_token=72dea7021454f100bc72154931cdd6e0a6eecd76; xqat=72dea7021454f100bc72154931cdd6e0a6eecd76; xq_r_token=ad070cd3d55cc70f02f135fb52765cfbe11fa994; xq_id_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1aWQiOi0xLCJpc3MiOiJ1YyIsImV4cCI6MTY3ODY2Njg3MCwiY3RtIjoxNjc3MTE1MTk0NzQ3LCJjaWQiOiJkOWQwbjRBWnVwIn0.c9dbDazJjd5yo45sU4aS-IyFRCvgOy7EBWfsEXlbcxeWMSOLpc8zKk9Qk5yia38fpwxfFKsNyML5z_3d5-DuAwWozyUe4IviqVhZem_Ze-Lt959qK4lvHagqeYU9GZ-TtCDQGIBDWvaiwNDyDRsJJhDeAru7oR_Ll_N0AiANvmjlBPYRu0BVIrK5TVErkNuBy6bv2gZlaZU2sAjZ22iBm3oLqBZdbfDhICkJgNzWV7S5oL1Pl3vuh2HVhSa7P2HVft7Q-UZITEgQ0vq8GkkRRc1as64klPy2iSBuE2XzjdFXfR1vfHl222lOeI9BYvJ-7JuhAsXvIZuMchNA1qU5sg; u=281677115225756; Hm_lvt_1db88642e346389874251b5a1eded6e3=1676595996,1677115226; Hm_lpvt_1db88642e346389874251b5a1eded6e3=1677115237' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36' 



## 公司盈利能力量化指标

1. 最直接的技术资产带来的收益
  1. 资产收益率 ROA = 净利润/ 平均总资产
  2. 权益收益率 ROC = 净利润/ 平均净资产
  3. 投资资本收益率 ROIC = 息税前利润*(1-所得税税率) / (固定资产+无形资产+流动资产-流动负债-现金)

2. 盈利能力的持续性
  1. 长期平均资产收益率8yr_ROA = 5-8年的 ROA 的几何平均数
  2. 长期平均权益收益率8yr_ROC = 5-8年的 ROC 的几何平均数
  3. 长期投资资本收益率8yr_ROIC = 5-8年的 ROIC 的几何平均数

3. 盈利能力的成长性
  1. 毛利率增长率 Margin Grouth = 5-8年的 GM(gross margin) 的几何平均数

4. 盈利能力的稳定性
  1. Margin Stability = Avg(GM)/SD(GM) 5-8年的毛利率的平均值与标准差的比值

5. 成长性和稳定性的选择
  1. 最大盈利水平MM = Max(Percentile(MS), Percentile(MG)), 取MS，MG 在样本总体中的百分位数中的大的一个。

6. 盈利能力分解
  1. ROE 净资产收益率 = 净利润/平均净资产 = 净利润/营业收入 * 营业收入/总资产 * 总资产/平均净资产
  2. 经营性资产收益率 = 营运收益 / 净营运资产 = 营业利润 / 营业收入 * 营业收入 / 平均净经营性资产
    1. 销售利润率 = 营业利润 / 营业收入， 具有均值回归的特性（体现了产品的定价能力，较高容易引来竞争者的加入，所有容易回归）
    2. 经营性资产周转率 = 营业收入 / 平均净经营性资产，具有较高的持续性（体现了公司的经营管理效率，通常不容易被模仿）
  3. 存货周转率 = 营业收入 / 平均存货，对快销零售行业比较有用，对于制造业，存货周转率的变化不会对公司的盈利能力产生太大的影响
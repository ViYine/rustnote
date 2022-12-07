import requests

k2url = " https://www.zsxg.cn/api/k/pc/API/KLine2"
h= {
    "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
}
d = "field[]=name&field[]=symbol&field[]=yclose&field[]=open&field[]=price&field[]=high&field[]=low&field[]=vol&symbol=600585.SH&start=-1&count=3000"

krsp = requests.post(k2url,headers=h,data=d)
kdata = krsp.json()["data"]

date = ["20211128", "20211205", "20211212", "20211219", "20211226", "20211230", "20220109", "20220116", "20220123", "20220127", "20220206", "20220213", "20220220", "20220227", "20220306", "20220313", "20220320", "20220327", "20220331", "20220410", "20220417", "20220424", "20220428", "20220508", "20220515", "20220522", "20220529", "20220605", "20220612", "20220619", "20220626", "20220703", "20220710", "20220717", "20220724", "20220731", "20220807", "20220814", "20220821", "20220828", "20220904", "20220908", "20220918", "20220925", "20220929", "20221009", "20221016", "20221023", "20221030", "20221106", "20221113", "20221120", "20221201"]

date_i = [int(x) for x in date]
print(len(date_i))
kdata_0_1 = [(x[0],x[1]) for x in kdata if x[0] > 20211128]
# to dict
kdata_0_1_dict = dict(kdata_0_1)
print(kdata_0_1_dict.keys())
print(kdata_0_1_dict.values())

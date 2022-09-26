use std::collections::HashMap;

use anyhow::Result;
use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};
use tokio::fs;
use url::Url;

use crate::ExtraArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffProfile {
    // 请求相关的profile 配置
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    // 响应中有需要skip 的阈，
    pub res: ResponseProfile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    // 在默认没有传值的时候，不进行序列化
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        with = "http_serde::header_map",
        skip_serializing_if = "HeaderMap::is_empty",
        default
    )]
    pub headers: HeaderMap,
    // 在默认没有传值的时候，不进行序列化
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}


impl DiffConfig {

    // 从文件读
    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    // 从字符串读
    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }

    // 给一个 profile name 返回一个profile config
    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}



impl DiffProfile {
    pub async fn diff(&self, args: ExtraArgs) -> Result<String> {
        // _args 是需要override 的参数（由用户通过命令行传入）
        // 从命令行拿到的参数，先合并到对应的：req，res
        // 然后 send request 得到具体的，响应内容
        // let res1 = &self.req1.send(&args).await?;
        // let res2 = &self.req2.send(&args).await?;
        // // 从响应内容中去除掉需要skip 的text，剩下需要进行 diff 比较的text
        // let text1 = res1.filter_text(&self.res).await?;
        // let text2 = res2.filter_text(&self.res).await?;

        // // 调用 similar 的 string 的diff 函数进行输出。
        // text_diff(text1, text2);
        println!("profile: {:?}", &self);
        println!("extra_prams: {:?}", &args);

        Ok("".to_string())

        // todo!()
    }
}

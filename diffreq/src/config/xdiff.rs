use crate::{util::text_diff, ExtraArgs};
use std::collections::HashMap;

use super::{is_default, ConfigLoad, ConfigValidate, GetProfile, RequestProfile};

use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl ResponseProfile {
    pub fn new(skip_headers: Vec<String>, skip_body: Vec<String>) -> Self {
        Self {
            skip_headers,
            skip_body,
        }
    }
}

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
    #[serde(skip_serializing_if = "is_default", default)]
    pub res: ResponseProfile,
}

// 直接使用公共的 config load trait 的实现，不需要重复写
impl ConfigLoad for DiffConfig {}

impl GetProfile for DiffConfig {
    // 关联类型为 DiffProfile
    type Profile = DiffProfile;
    fn get_profile(&self, name: &str) -> Option<&Self::Profile> {
        self.profiles.get(name)
    }
}

impl DiffConfig {
    pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
        Self { profiles }
    }
}

impl DiffProfile {
    pub fn new(req1: RequestProfile, req2: RequestProfile, res: ResponseProfile) -> Self {
        Self { req1, req2, res }
    }
    pub async fn diff(&self, args: ExtraArgs) -> Result<String> {
        // _args 是需要override 的参数（由用户通过命令行传入）
        // 从命令行拿到的参数，先合并到对应的：req，res
        // 然后 send request 得到具体的，响应内容
        let res1 = self.req1.send(&args).await?;
        let res2 = self.req2.send(&args).await?;
        // // 从响应内容中去除掉需要skip 的text，剩下需要进行 diff 比较的text
        let text1 = res1.filter_text(&self.res).await?;
        let text2 = res2.filter_text(&self.res).await?;
        // println!("{}", text1);
        // println!("text2: {}", text2);

        // // 调用 similar 的 string 的diff 函数进行输出。
        let output = text_diff(&text1, &text2)?;
        // println!("profile: {:?}", &self);
        // println!("extra_prams: {:?}", &args);
        // println!("{}", output);

        Ok(output)
        // 错误处理：针对，程序范围的无效的输入，配置等作出更好的出错的提示。

        // todo!()
    }
}

// validate for diff profile
impl ConfigValidate for DiffProfile {
    fn validate(&self) -> Result<()> {
        self.req1.validate().context("req1 config error")?;
        self.req2.validate().context("req2 config error")?;
        Ok(())
    }
}

// validate
impl ConfigValidate for DiffConfig {
    fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("validate prifile name: {}", name))?;
        }
        Ok(())
    }
}

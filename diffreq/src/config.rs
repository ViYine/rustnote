use std::collections::HashMap;

use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{util::text_diff, ExtraArgs, RequestProfile, ResponseProfile};

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
    #[serde(skip_serializing_if = "is_defalult", default)]
    pub res: ResponseProfile,
}

// 如果是default 值则不序列化
fn is_defalult<T: PartialEq + Default>(v: &T) -> bool {
    v == &T::default()
}

impl DiffConfig {
    pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
        Self { profiles }
    }
    // 从文件读
    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    // 从字符串读
    pub fn from_yaml(content: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }
    // validate
    fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("validate prifile name: {}", name))?;
        }
        Ok(())
    }

    // 给一个 profile name 返回一个profile config
    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
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

    fn validate(&self) -> Result<()> {
        self.req1.validate().context("req1 config error")?;
        self.req2.validate().context("req2 config error")?;
        Ok(())
    }
}

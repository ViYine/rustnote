use super::{ConfigLoad, ConfigValidate, GetProfile, RequestProfile};
use std::collections::HashMap;

use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, RequestProfile>,
}

// 直接使用公共的 config load trait 的实现，不需要重复写
impl ConfigLoad for RequestConfig {}

impl ConfigValidate for RequestConfig {
    fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .with_context(|| format!("profile: {}", name))?;
        }
        Ok(())
    }
}

impl GetProfile for RequestConfig {
    // 关联类型为 RequestProfile
    type Profile = RequestProfile;
    fn get_profile(&self, name: &str) -> Option<&Self::Profile> {
        self.profiles.get(name)
    }
}

impl RequestConfig {
    pub fn new(profiles: HashMap<String, RequestProfile>) -> Self {
        Self { profiles }
    }
}

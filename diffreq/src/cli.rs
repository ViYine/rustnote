use anyhow::{anyhow, Result};

use crate::ExtraArgs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    val: String,
}

pub fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key val par:{}", s))?
        .trim();
    let val = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key val par:{}", s))?
        .trim();

    let (key_type, key_s) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('#') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key val par")),
    };

    Ok(KeyVal {
        key: key_s.into(),
        val: val.into(),
        key_type,
    })
}

//  把从命令行解析出来的参数，转换成 extra_args 传递给，request 进行处理
impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Body => body.push((arg.key, arg.val)),
                KeyValType::Query => query.push((arg.key, arg.val)),
                KeyValType::Header => headers.push((arg.key, arg.val)),
            }
        }

        Self {
            headers,
            query,
            body,
        }
    }
}

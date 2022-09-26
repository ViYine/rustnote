use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::ExtraArgs;

/// Diff two http request and compare the difference of the response
#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Clone, Subcommand)]
#[non_exhaustive]
pub enum Action {
    /// Diff two API response based on the given profile
    Run(RunArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct RunArgs {
    /// Profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Override args, Could be used to override the query, headers,and body of the request
    /// For query parameters: use `-e key=value`
    /// For headers: use `-e %key=value`
    /// For body: use `-e #key=value`
    #[clap(short, long, value_parser=parse_key_val, number_of_values=1)]
    pub extra_params: Vec<KeyVal>,

    /// Configuration to be used
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

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

fn parse_key_val(s: &str) -> Result<KeyVal> {
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

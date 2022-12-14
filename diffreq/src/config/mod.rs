pub mod xdiff;
pub mod xreq;

use std::str::FromStr;

use crate::ExtraArgs;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Client, Method, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use string_builder::Builder;
use url::Url;

use crate::cli::parse_key_val;
use crate::cli::KeyVal;
use anyhow::Result;
use async_trait::async_trait;

use clap::{Parser, Subcommand};

use tokio::fs;
pub use xdiff::ResponseProfile;

// load config from yaml file or string trait
#[async_trait]
pub trait ConfigLoad
where
    Self: Sized + ConfigValidate + DeserializeOwned,
{
    /// load yaml config from file
    async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    /// load yaml config from string
    fn from_yaml(content: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(content)?;
        // 需要使用validate方法来检查配置是否合法，所以Self需要实现ConfigValidate trait
        config.validate()?;
        Ok(config)
    }
}

// validate config trait
pub trait ConfigValidate {
    fn validate(&self) -> Result<()>;
}

// get profile from config trait， 当前这个trait 的意义可能不大，会增加代码量
pub trait GetProfile {
    // 不同的config有不同的profile类型，所以这里用泛型
    type Profile;
    fn get_profile(&self, name: &str) -> Option<&Self::Profile>;
}

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
    /// Parse the given url and name into a profile output
    Parse,
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

// 如果是default 值则不序列化
fn is_default<T: PartialEq + Default>(v: &T) -> bool {
    v == &T::default()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    // 在默认没有传值的时候，不进行序列化
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        with = "http_serde::header_map",
        skip_serializing_if = "HeaderMap::is_empty",
        default
    )]
    pub headers: HeaderMap,
    // 在默认没有传值的时候，不进行序列化
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub body: Option<serde_json::Value>,
}

fn empty_json_value(val: &Option<serde_json::Value>) -> bool {
    val.as_ref().map_or(true, |v| {
        v.is_null() || (v.is_object() && v.as_object().unwrap().is_empty())
    })
}

// 对拿到的reqwest response 做了一次封装
#[derive(Debug)]
pub struct ResponseExt(Response);

impl FromStr for RequestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut url = Url::parse(s)?;
        let querys = url.query_pairs();
        let mut params = json!({});
        for (key, value) in querys {
            params[&*key] = value.parse()?
        }
        // url set query none
        url.set_query(None);
        Ok(RequestProfile::new(
            Method::GET,
            url,
            Some(params),
            HeaderMap::new(),
            None,
        ))
    }
}

impl RequestProfile {
    pub fn new(
        method: Method,
        url: Url,
        params: Option<serde_json::Value>,
        headers: HeaderMap,
        body: Option<serde_json::Value>,
    ) -> Self {
        Self {
            method,
            url,
            params,
            headers,
            body,
        }
    }
    pub async fn send(&self, args: &ExtraArgs) -> Result<ResponseExt> {
        // args merge to self
        let (query, header, body) = self.generate(args)?;
        // create client
        let cli = Client::new();
        // fill query, headers, and body
        let req = cli
            .request(self.method.clone(), self.url.clone())
            .query(&query)
            .headers(header)
            .body(body)
            .build()?;
        // send request
        let res = cli.execute(req).await?;

        // get response
        Ok(ResponseExt(res))
    }

    fn generate(&self, args: &ExtraArgs) -> Result<(serde_json::Value, HeaderMap, String)> {
        // 拿到配置的profile 里面的 query， headers，和body， 然后将args 里面的 query， headers，body 的值 添加到req 的profile 中
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut headers = self.headers.clone();
        let body = self.body.clone().unwrap_or_else(|| json!({}));
        // query add
        for (q_k, q_v) in &args.query {
            // parse 是从str 中转换成目标类型
            query[q_k] = q_v.parse()?;
        }
        // headers add
        for (h_k, h_v) in &args.headers {
            headers.insert(HeaderName::from_str(h_k)?, h_v.parse()?);
        }
        // body add and serialize to string
        // default add json serialize
        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        // 获取content_type
        let content_type = get_content_type(&headers);
        // 根据content_type 序列化body
        match content_type.as_deref() {
            Some("application/json") => Ok((query, headers, serde_json::to_string(&body)?)),
            Some("application/x-www-form-urlencoded" | "multipart/from-data") => {
                Ok((query, headers, serde_urlencoded::to_string(&body)?))
            }
            Some("text/plain" | "ad-bill-pb/base64") => Ok((query, headers, body.to_string())),
            // todo!() add other content-type support
            _ => Err(anyhow::anyhow!("unsupport application type")),
        }
    }

    pub(crate) fn validate(&self) -> Result<()> {
        let headers = self.headers.clone();
        let content_type = get_content_type(&headers);
        match content_type.as_deref() {
            Some(
                "application/json" | "application/x-www-form-urlencoded" | "multipart/from-data",
            ) => {
                if let Some(body) = &self.body {
                    if !body.is_object() {
                        return Err(anyhow::anyhow!(
                            "Body must be an object: but got \n{}\n",
                            serde_yaml::to_string(body)?
                        ));
                    }
                }
            }
            _ => {
                // body is string
                if let Some(body) = &self.body {
                    if !body.is_string() {
                        return Err(anyhow::anyhow!(
                            "Body must be an string: but got \n{}\n",
                            serde_yaml::to_string(body)?
                        ));
                    }
                }
            }
        }

        if let Some(params) = &self.params {
            if !params.is_object() {
                return Err(anyhow::anyhow!(
                    "Params must be an object: but got \n{}\n",
                    serde_yaml::to_string(params)?
                ));
            }
        }

        Ok(())
    }
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().unwrap().split(';').next())
        .map(|v| v.to_string())
}

impl ResponseExt {
    pub fn into_inner(self) -> Response {
        self.0
    }

    pub fn get_header_keys(self) -> Vec<String> {
        let res_headers = self.0.headers();
        res_headers.iter().map(|(k, _)| k.to_string()).collect()
    }
    pub async fn filter_text(self, res: &ResponseProfile) -> Result<String> {
        // ResponseExt 里面是原始的请求，需要skip 的 阈 在res 中指定了，所以需要返回，res 中不skip 的 key 的值
        let mut output_builder = Builder::default();
        // let first_line = format!("{:?} {}\r\n", self.0.version(), self.0.status());
        // output_builder.append(first_line);
        // let output = get_header_text(self.0.headers(), &res.skip_headers)?;
        // output_builder.append(output);
        output_builder.append(get_status_text(&self.0)?);
        output_builder.append(get_header_text(&self.0, &res.skip_headers)?);
        output_builder.append(get_body_text(self.0, &res.skip_body).await?);

        Ok(output_builder.string()?)
    }
}

pub fn get_status_text(res: &Response) -> Result<String> {
    Ok(format!("{:?} {}\r\n", res.version(), res.status()))
}

pub fn get_header_text(res: &Response, skip_header: &[String]) -> Result<String> {
    let mut output_builder = Builder::default();
    // let res_headers = self.0.headers();

    for (k, v) in res.headers().iter() {
        if skip_header.contains(&k.to_string()) {
            continue;
        }
        output_builder.append(format!("{}: {}\r\n", &k.as_str(), v.to_str()?));
    }
    output_builder.append("\r\n");
    Ok((output_builder.string())?)
}

pub async fn get_body_text(res: Response, skip_body: &[String]) -> Result<String> {
    let mut output_builder = Builder::default();
    let content_type = get_content_type(res.headers()).clone();
    let text = res.text().await?;
    // let ct = get_content_type(res_headers);
    // 根据content_type 反序列化body
    match content_type.as_deref() {
        Some("application/json") => {
            let output = filter_json_text(&text, skip_body)?;
            output_builder.append(output);
            Ok(output_builder.string()?)
        }

        _ => {
            // todo!() add other content-type support, now just return text
            Ok(text)
        }
    }
}

fn filter_json_text(text: &str, skip_body: &[String]) -> Result<String> {
    let mut out_val: serde_json::Value = serde_json::from_str(text)?;
    let res_val = out_val
        .as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("unsupport json type"))?;
    for key in skip_body {
        res_val.remove(key);
    }
    Ok(serde_json::to_string_pretty(&res_val)?)
}

#[cfg(test)]
mod tests {
    use mockito::Mock;

    use super::*;

    #[test]
    fn test_get_content_type() {
        // 内部的private 的方法，不要测试，因为其内部实现不稳定
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        assert_eq!(
            get_content_type(&headers),
            Some("application/json".to_string())
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        assert_eq!(
            get_content_type(&headers),
            Some("application/json".to_string())
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        assert_eq!(
            get_content_type(&headers),
            Some("application/json".to_string())
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        assert_eq!(
            get_content_type(&headers),
            Some("application/json".to_string())
        );
    }

    #[tokio::test]
    async fn request_profile_send_should_work() {
        let _m = mock_for_url("/todo?a=1&b=2", "Get", json!({"id": 1, "name": "todo"}));
        let url = format!("{}/todo", mockito::server_url());
        let req = RequestProfile::new(
            Method::GET,
            Url::parse(&url).unwrap(),
            Some(json!({"a": 1, "b": 2})),
            HeaderMap::new(),
            None,
        );
        let res = req.send(&Default::default()).await.unwrap().into_inner();
        assert_eq!(res.status(), 200);
    }

    fn mock_for_url(path_and_query: &str, method: &str, body: serde_json::Value) -> Mock {
        mockito::mock(method, path_and_query)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&body).unwrap())
            .create()
    }
}

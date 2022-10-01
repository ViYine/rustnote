use std::str::FromStr;

use crate::ExtraArgs;
use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Client, Method, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use string_builder::Builder;
use url::Url;

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

// 对拿到的reqwest response 做了一次封装
#[derive(Debug)]
pub struct ResponseExt(Response);

impl RequestProfile {
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
            // todo!() add other content-type support
            _ => Err(anyhow::anyhow!("unsupport application type")),
        }
    }
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().unwrap().split(';').next())
        .map(|v| v.to_string())
}

impl ResponseExt {
    pub async fn filter_text(self, res: &ResponseProfile) -> Result<String> {
        // ResponseExt 里面是原始的请求，需要skip 的 阈 在res 中指定了，所以需要返回，res 中不skip 的 key 的值
        let mut output_builder = Builder::default();
        let first_line = format!("{:?} {}\r\n", self.0.version(), self.0.status());
        output_builder.append(first_line);
        let output = get_header_text(self.0.headers(), &res.skip_headers)?;
        output_builder.append(output);

        let content_type = get_content_type(self.0.headers()).clone();
        let text = self.0.text().await?;
        // let ct = get_content_type(res_headers);
        // 根据content_type 反序列化body
        match content_type.as_deref() {
            Some("application/json") => {
                let output = filter_json_text(&text, &res.skip_body)?;
                output_builder.append(output);
                Ok(output_builder.string()?)
            }
            // Some("application/x-www-form-urlencoded" | "multipart/from-data") => {
            //     todo!()
            // },
            // todo!() add other content-type support
            // _ => return Err(anyhow::anyhow!("unsupport response application type")),
            _ => {
                // todo!() add other content-type support, now just return text
                Ok(text)
            }
        }
    }
}

fn get_header_text(headers: &HeaderMap, skip_header: &[String]) -> Result<String> {
    let mut output_builder = Builder::default();
    // let res_headers = self.0.headers();

    for (k, v) in headers.iter() {
        if skip_header.contains(&k.to_string()) {
            continue;
        }
        output_builder.append(format!("{}: {}\r\n", &k.as_str(), v.to_str()?));
    }
    output_builder.append("\r\n");
    Ok((output_builder.string())?)
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

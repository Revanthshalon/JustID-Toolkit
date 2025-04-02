#![allow(unused)]

mod test;

use std::collections::HashMap;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::PaginationTrait;

#[derive(Debug, Deserialize)]
pub struct TokenPaginationParams {
    pub page_token: Option<String>,
    #[serde(default = "default_page_size")]
    #[serde(deserialize_with = "deserialize_page_size")]
    pub page_size: i32,
}

fn default_page_size() -> i32 {
    250
}

fn deserialize_page_size<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let page_size: i32 = match Option::<String>::deserialize(deserializer)? {
        Some(page_size_string) => page_size_string.parse().unwrap_or(250),
        None => 250,
    };
    Ok(page_size.clamp(1, 1000))
}

pub struct TokenPagination;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub offset: i32,
    pub version: i32,
}

impl TokenPagination {
    pub fn encode(offset: i32) -> String {
        let token = Token { offset, version: 1 };
        let token_str = json!(token).to_string();
        URL_SAFE_NO_PAD.encode(token_str)
    }

    pub fn decode(token: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let decoded = URL_SAFE_NO_PAD.decode(token)?;
        let decode_str = String::from_utf8(decoded)?;
        let token: Token = serde_json::from_str(&decode_str)?;
        let offset = token.offset;
        Ok(offset)
    }
}

impl PaginationTrait for TokenPagination {
    fn header(base_url: &str, rel: &str, items_per_page: i32, offset: i32) -> String {
        format!(
            "<{}?page_size={}&page_token={}>; rel=\"{}\"",
            base_url,
            items_per_page,
            Self::encode(offset),
            rel
        )
    }
}

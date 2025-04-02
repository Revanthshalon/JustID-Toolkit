#![allow(unused)]

mod test;

use std::collections::HashMap;

use serde::Deserialize;

use super::PaginationTrait;

#[derive(Debug, Deserialize)]
pub struct PagePaginationParams {
    #[serde(default = "default_page")]
    #[serde(deserialize_with = "deserialize_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    #[serde(deserialize_with = "deserialize_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    0
}
fn default_per_page() -> i32 {
    250
}

fn deserialize_page<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let page: i32 = match Option::<String>::deserialize(deserializer)? {
        Some(page_string) => page_string.parse().unwrap_or(0),
        None => 0,
    };
    Ok(page.max(0))
}

fn deserialize_per_page<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let items_per_page: i32 = match Option::<String>::deserialize(deserializer)? {
        Some(items_per_page_string) => items_per_page_string.parse().unwrap_or(250),
        None => 250,
    };
    Ok(items_per_page.clamp(1, 1000))
}

pub struct PagePagination;

impl PaginationTrait for PagePagination {
    fn header(base_url: &str, rel: &str, limit: i32, offset: i32) -> String {
        format!(
            "<{}?per_page={}&page={}>; rel=\"{}\"",
            base_url,
            limit,
            offset / limit,
            rel
        )
    }
}

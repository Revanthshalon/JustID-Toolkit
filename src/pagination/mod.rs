#![allow(unused)]

mod page_pagination;
mod token_pagination;

use std::collections::HashMap;

pub use self::page_pagination::{PagePagination, PagePaginationParams};
pub use self::token_pagination::{TokenPagination, TokenPaginationParams};

pub trait PaginationTrait {
    fn header(base_url: &str, rel: &str, items_per_page: i32, offset: i32) -> String;
    fn pagination_header(
        total: i32,
        page: i32,
        items_per_page: i32,
        base_url: &str,
    ) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        let items_per_page = if items_per_page <= 0 {
            1
        } else {
            items_per_page
        };
        let offset = page * items_per_page;

        let last_offset = if total % items_per_page == 0 {
            total - items_per_page
        } else {
            (total / items_per_page) * items_per_page
        };

        headers.insert("X-Total-Count".to_string(), total.to_string());

        let mut links = Vec::new();

        if offset >= last_offset {
            if total == 0 {
                links.push(Self::header(base_url, "first", items_per_page, 0));
                links.push(Self::header(
                    base_url,
                    "next",
                    items_per_page,
                    ((offset / items_per_page) + 1) * items_per_page,
                ));
                links.push(Self::header(
                    base_url,
                    "prev",
                    items_per_page,
                    ((offset / items_per_page) - 1) * items_per_page,
                ));
            } else if total <= items_per_page {
                links.push(Self::header(base_url, "first", total, 0));
            } else {
                links.push(Self::header(base_url, "first", items_per_page, 0));
                links.push(Self::header(
                    base_url,
                    "prev",
                    items_per_page,
                    last_offset - items_per_page,
                ));
            }
        } else if offset < items_per_page {
            links.push(Self::header(
                base_url,
                "next",
                items_per_page,
                items_per_page,
            ));
            links.push(Self::header(base_url, "last", items_per_page, last_offset));
        } else {
            links.push(Self::header(base_url, "first", items_per_page, 0));
            links.push(Self::header(
                base_url,
                "next",
                items_per_page,
                ((offset / items_per_page) + 1) * items_per_page,
            ));
            links.push(Self::header(
                base_url,
                "prev",
                items_per_page,
                ((offset / items_per_page) - 1) * items_per_page,
            ));
            links.push(Self::header(base_url, "last", items_per_page, last_offset));
        }

        if !links.is_empty() {
            headers.insert("Link".to_string(), links.join(","));
        }

        headers
    }
}

use super::{PagePagination, PagePaginationParams};

use crate::pagination::PaginationTrait;

#[test]
pub fn test_deseriazlize_page_pagination_params() {
    // Test normal usecase
    let query = "page=2&per_page=30";
    let params: PagePaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page, 2);
    assert_eq!(params.per_page, 30);

    // Test default usecase
    let query = "";
    let params: PagePaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page, 0);
    assert_eq!(params.per_page, 250);

    // Test invalid values
    let query = "page=&per_page=";
    let params: PagePaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page, 0);
    assert_eq!(params.per_page, 250);

    // Test invalue values
    let query = "page=invalid&per_page=invalid";
    let params: PagePaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page, 0);
    assert_eq!(params.per_page, 250);

    // Test clamping
    let query = "page=-5&per_page=5000";
    let params: PagePaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page, 0);
    assert_eq!(params.per_page, 1000);

    // Test minumum page
    let query = "per_page=0";
    let params: PagePaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page, 0);
    assert_eq!(params.per_page, 1);
}

#[test]
fn test_header_function() {
    let base_url = "https://api.example.com/items";
    let result = PagePagination::header(base_url, "next", 20, 40);
    assert_eq!(
        result,
        "<https://api.example.com/items?per_page=20&page=2>; rel=\"next\""
    )
}

#[test]
fn test_pagination_header_first_page() {
    let base_url = "https://api.example.com/items";
    let headers = PagePagination::pagination_header(97, 0, 20, base_url);

    assert_eq!(headers.get("X-Total-Count"), Some(&"97".to_string()));

    let link = headers.get("Link").unwrap();
    assert!(link.contains("rel=\"next\""));
    assert!(link.contains("rel=\"last\""));
    assert!(!link.contains("rel=\"prev\""));
    assert!(!link.contains("rel=\"first\""));

    // Check specific page values
    assert!(link.contains("page=1")); // next page
    assert!(link.contains("page=4")); // last page
}

#[test]
fn test_pagination_header_middle_page() {
    let base_url = "https://api.example.com/items";
    let headers = PagePagination::pagination_header(97, 2, 20, base_url);

    assert_eq!(headers.get("X-Total-Count"), Some(&"97".to_string()));

    let link = headers.get("Link").unwrap();
    assert!(link.contains("rel=\"first\""));
    assert!(link.contains("rel=\"next\""));
    assert!(link.contains("rel=\"prev\""));
    assert!(link.contains("rel=\"last\""));

    // Check specific page values
    assert!(link.contains("page=0")); // first page
    assert!(link.contains("page=3")); // next page
    assert!(link.contains("page=1")); // prev page
    assert!(link.contains("page=4")); // last page
}

#[test]
fn test_pagination_header_last_page() {
    let base_url = "https://api.example.com/items";
    // Page 4 with 20 items per page (97 total items)
    let headers = PagePagination::pagination_header(97, 4, 20, base_url);

    assert_eq!(headers.get("X-Total-Count"), Some(&"97".to_string()));

    let link = headers.get("Link").unwrap();
    assert!(link.contains("rel=\"first\""));
    assert!(!link.contains("rel=\"next\""));
    assert!(link.contains("rel=\"prev\""));
    assert!(!link.contains("rel=\"last\""));

    // Check specific page values
    assert!(link.contains("page=0")); // first page
    assert!(link.contains("page=3")); // prev page
}

#[test]
fn test_pagination_header_empty_results() {
    let base_url = "https://api.example.com/items";
    let headers = PagePagination::pagination_header(0, 0, 20, base_url);

    assert_eq!(headers.get("X-Total-Count"), Some(&"0".to_string()));

    let link = headers.get("Link").unwrap();
    // For empty results, Go implementation has specific behavior
    assert!(link.contains("rel=\"first\""));
    assert!(link.contains("rel=\"next\""));
    assert!(link.contains("rel=\"prev\""));
}

#[test]
fn test_pagination_header_single_page() {
    let base_url = "https://api.example.com/items";
    // 15 total items with 20 per page - should fit on single page
    let headers = PagePagination::pagination_header(15, 0, 20, base_url);

    assert_eq!(headers.get("X-Total-Count"), Some(&"15".to_string()));

    let link = headers.get("Link").unwrap();
    // For single page of results at last page
    assert!(!link.contains("rel=\"prev\""));
    assert!(!link.contains("rel=\"next\""));
    assert!(link.contains("rel=\"first\""));
    assert!(!link.contains("rel=\"last\""));
}

#[test]
fn test_edge_cases() {
    // Test with very small per_page
    let headers = PagePagination::pagination_header(100, 5, 1, "https://api.example.com/items");
    assert_eq!(headers.get("X-Total-Count"), Some(&"100".to_string()));

    // Test with negative page (should be handled as 0)
    let headers = PagePagination::pagination_header(100, -1, 20, "https://api.example.com/items");
    let link = headers.get("Link").unwrap();
    // Should be treated as page 0
    assert!(link.contains("page=1")); // next page
    assert!(!link.contains("rel=\"prev\"")); // no prev on first page

    // Test with equal total and per_page
    let headers = PagePagination::pagination_header(20, 0, 20, "https://api.example.com/items");
    let link = headers.get("Link").unwrap();
    assert!(!link.contains("rel=\"last\"")); // No last link needed
}

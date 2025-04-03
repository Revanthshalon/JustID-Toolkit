use base64::Engine;

use super::{Token, TokenPagination, TokenPaginationParams};

use crate::pagination::PaginationTrait;
use std::error::Error;

#[test]
pub fn test_deserialize_token_pagination_params() {
    // Test normal usecase
    let query = "page_token=sometoken&page_size=30";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, Some("sometoken".to_string()));
    assert_eq!(params.page_size, 30);

    // Test default usecase
    let query = "";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, None);
    assert_eq!(params.page_size, 250);

    // Test with only page token
    let query = "page_token=sometoken";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, Some("sometoken".to_string()));
    assert_eq!(params.page_size, 250);

    // Test empty values
    let query = "page_token=&page_size=";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, Some("".to_string()));
    assert_eq!(params.page_size, 250);

    // Test invalid page size
    let query = "page_token=sometoken&page_size=invalid";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, Some("sometoken".to_string()));
    assert_eq!(params.page_size, 250);

    // Test clamping
    let query = "page_size=5000";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, None);
    assert_eq!(params.page_size, 1000);

    // Test minimum page size
    let query = "page_size=0";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, None);
    assert_eq!(params.page_size, 1);

    // Test negative page size (should be clamped to 1)
    let query = "page_size=-10";
    let params: TokenPaginationParams = serde_urlencoded::from_str(&query).unwrap();

    assert_eq!(params.page_token, None);
    assert_eq!(params.page_size, 1);
}
#[test]
fn test_token_encoding_decoding() {
    // Test encoding and decoding with positive offset
    let offset = 100;
    let encoded = TokenPagination::encode(offset);
    let decoded = TokenPagination::decode(&encoded).unwrap();
    assert_eq!(decoded, offset);

    // Test with zero offset
    let offset = 0;
    let encoded = TokenPagination::encode(offset);
    let decoded = TokenPagination::decode(&encoded).unwrap();
    assert_eq!(decoded, offset);

    // Test with negative offset
    let offset = -50;
    let encoded = TokenPagination::encode(offset);
    let decoded = TokenPagination::decode(&encoded).unwrap();
    assert_eq!(decoded, offset);

    // Test with large offset
    let offset = 1_000_000;
    let encoded = TokenPagination::encode(offset);
    let decoded = TokenPagination::decode(&encoded).unwrap();
    assert_eq!(decoded, offset);
}

#[test]
fn test_token_decoding_failure() {
    // Test with invalid base64
    let invalid_token = "this is not base64!";
    let result = TokenPagination::decode(invalid_token);
    assert!(result.is_err());

    // Test with valid base64 but invalid JSON
    let invalid_json = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode("invalid json");
    let result = TokenPagination::decode(&invalid_json);
    assert!(result.is_err());

    // Test with valid base64 and JSON but wrong structure
    let invalid_structure =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode("{\"wrong\":123}");
    let result = TokenPagination::decode(&invalid_structure);
    assert!(result.is_err());
}

#[test]
fn test_header_function() {
    let base_url = "https://api.example.com/items";
    let result = TokenPagination::header(base_url, "next", 20, 40);

    // The token will be an encoded version of the JSON for Token{offset:40, version:1}
    let encoded_token = TokenPagination::encode(40);
    let expected = format!(
        "<https://api.example.com/items?page_size=20&page_token={}>; rel=\"next\"",
        encoded_token
    );

    assert_eq!(result, expected);
}

#[test]
fn test_token_roundtrip() {
    // Test that tokens maintain the same data through a full encode/decode cycle
    let offsets = vec![0, 1, 10, 100, 1000, -1, -100];

    for offset in offsets {
        let encoded = TokenPagination::encode(offset);
        let decoded = TokenPagination::decode(&encoded).unwrap();
        assert_eq!(
            decoded, offset,
            "Token roundtrip failed for offset {}",
            offset
        );
    }
}

#[test]
fn test_token_version_compatibility() {
    // Create a token with the current version
    let offset = 42;
    let token = Token { offset, version: 1 };
    let token_str = serde_json::to_string(&token).unwrap();
    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(token_str);

    // It should be decodable
    let decoded = TokenPagination::decode(&encoded).unwrap();
    assert_eq!(decoded, offset);

    // If in the future we decide to change the format, we might add more tests here
    // to check backward compatibility
}

#[test]
fn test_edge_cases() {
    // Test encoding/decoding with extreme values
    let extreme_cases = vec![i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX - 1, i32::MAX];

    for value in extreme_cases {
        let encoded = TokenPagination::encode(value);
        let decoded = TokenPagination::decode(&encoded).unwrap();
        assert_eq!(decoded, value, "Failed for extreme value {}", value);
    }

    // Test empty token string
    let result = TokenPagination::decode("");
    assert!(result.is_err());
}

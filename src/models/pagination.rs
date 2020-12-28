/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Pagination : Information about the current pagination. The keys previous_page, next_page, last_page, and total_entries may be null when on the first page, last page, or when the total number of entries is unknown



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    /// The current page number
    #[serde(rename = "page")]
    pub page: i32,
    /// The number of entries per page
    #[serde(rename = "per_page")]
    pub per_page: i32,
    /// The previous page number
    #[serde(rename = "previous_page", skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<i32>,
    /// The next page number
    #[serde(rename = "next_page", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<i32>,
    /// The last page number
    #[serde(rename = "last_page", skip_serializing_if = "Option::is_none")]
    pub last_page: Option<i32>,
    /// The total number of entries
    #[serde(rename = "total_entries", skip_serializing_if = "Option::is_none")]
    pub total_entries: Option<i32>,
}

impl Pagination {
    /// Information about the current pagination. The keys previous_page, next_page, last_page, and total_entries may be null when on the first page, last page, or when the total number of entries is unknown
    pub fn new(page: i32, per_page: i32) -> Pagination {
        Pagination {
            page,
            per_page,
            previous_page: None,
            next_page: None,
            last_page: None,
            total_entries: None,
        }
    }
}



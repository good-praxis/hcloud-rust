/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceNetworkRequest : Request for PUT https://api.hetzner.cloud/v1/networks/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceNetworkRequest {
    /// New network name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl ReplaceNetworkRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/networks/{id}
    pub fn new() -> ReplaceNetworkRequest {
        ReplaceNetworkRequest {
            name: None,
            labels: None,
        }
    }
}



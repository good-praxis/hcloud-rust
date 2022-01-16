/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeIpRangeOfNetworkRequest : Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeIpRangeOfNetworkRequest {
    /// The new prefix for the whole Network
    #[serde(rename = "ip_range")]
    pub ip_range: String,
}

impl ChangeIpRangeOfNetworkRequest {
    /// Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_ip_range
    pub fn new(ip_range: String) -> ChangeIpRangeOfNetworkRequest {
        ChangeIpRangeOfNetworkRequest {
            ip_range,
        }
    }
}



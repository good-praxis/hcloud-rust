/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeNetworkProtectionRequest : Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeNetworkProtectionRequest {
    /// If true, prevents the Network from being deleted
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
}

impl ChangeNetworkProtectionRequest {
    /// Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_protection
    pub fn new() -> ChangeNetworkProtectionRequest {
        ChangeNetworkProtectionRequest {
            delete: None,
        }
    }
}



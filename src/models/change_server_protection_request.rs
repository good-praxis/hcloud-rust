/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeServerProtectionRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeServerProtectionRequest {
    /// If true, prevents the Server from being deleted (currently delete and rebuild attribute needs to have the same value)
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    /// If true, prevents the Server from being rebuilt (currently delete and rebuild attribute needs to have the same value)
    #[serde(rename = "rebuild", skip_serializing_if = "Option::is_none")]
    pub rebuild: Option<bool>,
}

impl ChangeServerProtectionRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_protection
    pub fn new() -> ChangeServerProtectionRequest {
        ChangeServerProtectionRequest {
            delete: None,
            rebuild: None,
        }
    }
}



/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateFloatingIpRequest : Request for POST https://api.hetzner.cloud/v1/floating_ips



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFloatingIpRequest {
    /// Floating IP type
    #[serde(rename = "type")]
    pub _type: Type,
    /// Server to assign the Floating IP to
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<i32>,
    /// Home Location (routing is optimized for that Location). Only optional if Server argument is passed.
    #[serde(rename = "home_location", skip_serializing_if = "Option::is_none")]
    pub home_location: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl CreateFloatingIpRequest {
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips
    pub fn new(_type: Type) -> CreateFloatingIpRequest {
        CreateFloatingIpRequest {
            _type,
            server: None,
            home_location: None,
            description: None,
            name: None,
            labels: None,
        }
    }
}

/// Floating IP type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}


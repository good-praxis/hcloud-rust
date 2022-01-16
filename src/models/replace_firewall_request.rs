/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceFirewallRequest : Request for PUT https://api.hetzner.cloud/v1/firewalls/{id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceFirewallRequest {
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// New Firewall name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReplaceFirewallRequest {
    /// Request for PUT https://api.hetzner.cloud/v1/firewalls/{id}
    pub fn new() -> ReplaceFirewallRequest {
        ReplaceFirewallRequest {
            labels: None,
            name: None,
        }
    }
}



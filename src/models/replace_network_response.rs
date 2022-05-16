/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceNetworkResponse : Response to PUT https://api.hetzner.cloud/v1/networks/{id}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceNetworkResponse {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<Box<crate::models::Network>>,
}

impl ReplaceNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to PUT https://api.hetzner.cloud/v1/networks/{id}
    pub fn new() -> ReplaceNetworkResponse {
        ReplaceNetworkResponse { network: None }
    }
}

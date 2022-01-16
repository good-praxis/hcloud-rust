/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListNetworksResponse : Response to GET https://api.hetzner.cloud/v1/networks



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListNetworksResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
    #[serde(rename = "networks")]
    pub networks: Vec<crate::models::Network>,
}

impl ListNetworksResponse {
    /// Response to GET https://api.hetzner.cloud/v1/networks
    pub fn new(networks: Vec<crate::models::Network>) -> ListNetworksResponse {
        ListNetworksResponse {
            meta: None,
            networks,
        }
    }
}



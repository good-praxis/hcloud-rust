/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeleteRouteFromNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_route



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteRouteFromNetworkResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl DeleteRouteFromNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_route
    pub fn new(action: crate::models::Action) -> DeleteRouteFromNetworkResponse {
        DeleteRouteFromNetworkResponse {
            action,
        }
    }
}



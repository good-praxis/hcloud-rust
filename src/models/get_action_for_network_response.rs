/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForNetworkResponse : Response to GET https://api.hetzner.cloud/v1/networks/{id}/actions/{action_id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetActionForNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl GetActionForNetworkResponse {
    /// Response to GET https://api.hetzner.cloud/v1/networks/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForNetworkResponse {
        GetActionForNetworkResponse {
            action: Box::new(action),
        }
    }
}



/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActionResponse : Response to GET https://api.hetzner.cloud/v1/actions/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl GetActionResponse {
    /// Response to GET https://api.hetzner.cloud/v1/actions/{id}
    pub fn new(action: crate::models::Action) -> GetActionResponse {
        GetActionResponse {
            action,
        }
    }
}



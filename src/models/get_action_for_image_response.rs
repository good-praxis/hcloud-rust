/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForImageResponse : Response to GET https://api.hetzner.cloud/v1/images/{id}/actions/{action_id}



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetActionForImageResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl GetActionForImageResponse {
    /// Response to GET https://api.hetzner.cloud/v1/images/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForImageResponse {
        GetActionForImageResponse {
            action: Box::new(action),
        }
    }
}



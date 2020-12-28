/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeleteServiceResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteServiceResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl DeleteServiceResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service
    pub fn new(action: crate::models::Action) -> DeleteServiceResponse {
        DeleteServiceResponse {
            action,
        }
    }
}



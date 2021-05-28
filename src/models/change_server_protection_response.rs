/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeServerProtectionResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeServerProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeServerProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeServerProtectionResponse {
        ChangeServerProtectionResponse {
            action: Box::new(action),
        }
    }
}



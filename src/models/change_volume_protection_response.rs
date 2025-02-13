/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeVolumeProtectionResponse : Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeVolumeProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeVolumeProtectionResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/volumes/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeVolumeProtectionResponse {
        ChangeVolumeProtectionResponse {
            action: Box::new(action),
        }
    }
}



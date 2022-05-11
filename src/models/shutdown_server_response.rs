/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ShutdownServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/shutdown

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShutdownServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ShutdownServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/shutdown
    pub fn new(action: crate::models::Action) -> ShutdownServerResponse {
        ShutdownServerResponse {
            action: Box::new(action),
        }
    }
}

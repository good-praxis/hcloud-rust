/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeReverseDnsEntryForFloatingIpResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForFloatingIpResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeReverseDnsEntryForFloatingIpResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr
    pub fn new(action: crate::models::Action) -> ChangeReverseDnsEntryForFloatingIpResponse {
        ChangeReverseDnsEntryForFloatingIpResponse {
            action: Box::new(action),
        }
    }
}

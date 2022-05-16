/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListSshKeysResponse : Response to GET https://api.hetzner.cloud/v1/ssh_keys

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSshKeysResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
    #[serde(rename = "ssh_keys")]
    pub ssh_keys: Vec<crate::models::SshKey>,
}

impl ListSshKeysResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/ssh_keys
    pub fn new(ssh_keys: Vec<crate::models::SshKey>) -> ListSshKeysResponse {
        ListSshKeysResponse {
            meta: None,
            ssh_keys,
        }
    }
}

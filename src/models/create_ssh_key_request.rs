/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateSshKeyRequest : Request for POST https://api.hetzner.cloud/v1/ssh_keys



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSshKeyRequest {
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Name of the SSH key
    #[serde(rename = "name")]
    pub name: String,
    /// Public key
    #[serde(rename = "public_key")]
    pub public_key: String,
}

impl CreateSshKeyRequest {
    /// Request for POST https://api.hetzner.cloud/v1/ssh_keys
    pub fn new(name: String, public_key: String) -> CreateSshKeyRequest {
        CreateSshKeyRequest {
            labels: None,
            name,
            public_key,
        }
    }
}



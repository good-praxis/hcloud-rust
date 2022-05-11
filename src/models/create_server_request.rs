/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateServerRequest : Request for POST https://api.hetzner.cloud/v1/servers

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServerRequest {
    /// Auto-mount Volumes after attach
    #[serde(rename = "automount", skip_serializing_if = "Option::is_none")]
    pub automount: Option<bool>,
    /// ID or name of Datacenter to create Server in (must not be used together with location)
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// Firewalls which should be applied on the Server's public network interface at creation time
    #[serde(rename = "firewalls", skip_serializing_if = "Option::is_none")]
    pub firewalls: Option<Vec<crate::models::CreateServerRequestFirewalls>>,
    /// ID or name of the Image the Server is created from
    #[serde(rename = "image")]
    pub image: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// ID or name of Location to create Server in (must not be used together with datacenter)
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Name of the Server to create (must be unique per Project and a valid hostname as per RFC 1123)
    #[serde(rename = "name")]
    pub name: String,
    /// Network IDs which should be attached to the Server private network interface at the creation time
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<i32>>,
    /// ID of the Placement Group the server should be in
    #[serde(rename = "placement_group", skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<i32>,
    /// ID or name of the Server type this Server should be created with
    #[serde(rename = "server_type")]
    pub server_type: String,
    /// SSH key IDs (`integer`) or names (`string`) which should be injected into the Server at creation time
    #[serde(rename = "ssh_keys", skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    /// Start Server right after creation. Defaults to true.
    #[serde(rename = "start_after_create", skip_serializing_if = "Option::is_none")]
    pub start_after_create: Option<bool>,
    /// Cloud-Init user data to use during Server creation. This field is limited to 32KiB.
    #[serde(rename = "user_data", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Volume IDs which should be attached to the Server at the creation time. Volumes must be in the same Location.
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<i32>>,
}

impl CreateServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers
    pub fn new(image: String, name: String, server_type: String) -> CreateServerRequest {
        CreateServerRequest {
            automount: None,
            datacenter: None,
            firewalls: None,
            image,
            labels: None,
            location: None,
            name,
            networks: None,
            placement_group: None,
            server_type,
            ssh_keys: None,
            start_after_create: None,
            user_data: None,
            volumes: None,
        }
    }
}

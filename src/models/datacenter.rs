/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Datacenter : Datacenter this Server is located at



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Datacenter {
    /// ID of the Datacenter
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique identifier of the Datacenter
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the Datacenter
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "location")]
    pub location: crate::models::Location,
    #[serde(rename = "server_types")]
    pub server_types: crate::models::DatacenterServerTypes,
}

impl Datacenter {
    /// Datacenter this Server is located at
    pub fn new(id: i32, name: String, description: String, location: crate::models::Location, server_types: crate::models::DatacenterServerTypes) -> Datacenter {
        Datacenter {
            id,
            name,
            description,
            location,
            server_types,
        }
    }
}



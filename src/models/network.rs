/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Network {
    /// Point in time when the Network was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// ID of the Network
    #[serde(rename = "id")]
    pub id: i32,
    /// IPv4 prefix of the whole Network
    #[serde(rename = "ip_range")]
    pub ip_range: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Array of IDs of Load Balancers attached to this Network
    #[serde(rename = "load_balancers", skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<i32>>,
    /// Name of the Network
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::Protection>,
    /// Array of routes set in this Network
    #[serde(rename = "routes")]
    pub routes: Vec<crate::models::Route>,
    /// Array of IDs of Servers attached to this Network
    #[serde(rename = "servers")]
    pub servers: Vec<i32>,
    /// Array subnets allocated in this Network
    #[serde(rename = "subnets")]
    pub subnets: Vec<crate::models::SubnetWithGateway>,
}

impl Network {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        created: String,
        id: i32,
        ip_range: String,
        labels: ::std::collections::HashMap<String, String>,
        name: String,
        protection: crate::models::Protection,
        routes: Vec<crate::models::Route>,
        servers: Vec<i32>,
        subnets: Vec<crate::models::SubnetWithGateway>,
    ) -> Network {
        Network {
            created,
            id,
            ip_range,
            labels,
            load_balancers: None,
            name,
            protection: Box::new(protection),
            routes,
            servers,
            subnets,
        }
    }
}

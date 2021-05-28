/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Network {
    /// ID of the Network
    #[serde(rename = "id")]
    pub id: i32,
    /// Name of the Network
    #[serde(rename = "name")]
    pub name: String,
    /// IPv4 prefix of the whole Network
    #[serde(rename = "ip_range")]
    pub ip_range: String,
    /// Array subnets allocated in this Network
    #[serde(rename = "subnets")]
    pub subnets: Vec<crate::models::SubnetWithGateway>,
    /// Array of routes set in this Network
    #[serde(rename = "routes")]
    pub routes: Vec<crate::models::Route>,
    /// Array of IDs of Servers attached to this Network
    #[serde(rename = "servers")]
    pub servers: Vec<i32>,
    /// Array of IDs of Load Balancers attached to this Network
    #[serde(rename = "load_balancers", skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<i32>>,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::Protection>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Point in time when the Network was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
}

impl Network {
    pub fn new(id: i32, name: String, ip_range: String, subnets: Vec<crate::models::SubnetWithGateway>, routes: Vec<crate::models::Route>, servers: Vec<i32>, protection: crate::models::Protection, labels: ::std::collections::HashMap<String, String>, created: String) -> Network {
        Network {
            id,
            name,
            ip_range,
            subnets,
            routes,
            servers,
            load_balancers: None,
            protection: Box::new(protection),
            labels,
            created,
        }
    }
}



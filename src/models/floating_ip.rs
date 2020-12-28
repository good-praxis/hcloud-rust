/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatingIp {
    /// ID of the Floating IP
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique name of the Floating IP
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the Floating IP
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// IP address of the Floating IP
    #[serde(rename = "ip")]
    pub ip: String,
    /// Type of the Floating IP
    #[serde(rename = "type")]
    pub _type: Type,
    /// ID of the Server the Floating IP is assigned to, null if it is not assigned at all
    #[serde(rename = "server")]
    pub server: Option<i32>,
    /// Array of reverse DNS entries
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: Vec<crate::models::DnsPtr>,
    #[serde(rename = "home_location")]
    pub home_location: crate::models::Location,
    /// Whether the IP is blocked
    #[serde(rename = "blocked")]
    pub blocked: bool,
    #[serde(rename = "protection")]
    pub protection: crate::models::FloatingIpProtection,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Point in time when the Floating IP was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
}

impl FloatingIp {
    pub fn new(id: i32, name: String, description: Option<String>, ip: String, _type: Type, server: Option<i32>, dns_ptr: Vec<crate::models::DnsPtr>, home_location: crate::models::Location, blocked: bool, protection: crate::models::FloatingIpProtection, labels: ::std::collections::HashMap<String, String>, created: String) -> FloatingIp {
        FloatingIp {
            id,
            name,
            description,
            ip,
            _type,
            server,
            dns_ptr,
            home_location,
            blocked,
            protection,
            labels,
            created,
        }
    }
}

/// Type of the Floating IP
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}


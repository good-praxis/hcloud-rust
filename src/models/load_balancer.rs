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
pub struct LoadBalancer {
    #[serde(rename = "algorithm")]
    pub algorithm: Box<crate::models::LoadBalancerAlgorithm>,
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    /// Free Traffic for the current billing period in bytes
    #[serde(rename = "included_traffic")]
    pub included_traffic: i64,
    /// Inbound Traffic for the current billing period in bytes
    #[serde(rename = "ingoing_traffic")]
    pub ingoing_traffic: Option<i64>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    #[serde(rename = "load_balancer_type")]
    pub load_balancer_type: Box<crate::models::LoadBalancerType>,
    #[serde(rename = "location")]
    pub location: Box<crate::models::Location>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    /// Outbound Traffic for the current billing period in bytes
    #[serde(rename = "outgoing_traffic")]
    pub outgoing_traffic: Option<i64>,
    /// Private networks information
    #[serde(rename = "private_net")]
    pub private_net: Vec<crate::models::LoadBalancerPrivateNet>,
    #[serde(rename = "protection")]
    pub protection: Box<crate::models::Protection>,
    #[serde(rename = "public_net")]
    pub public_net: Box<crate::models::LoadBalancerPublicNet>,
    /// List of services that belong to this Load Balancer
    #[serde(rename = "services")]
    pub services: Vec<crate::models::LoadBalancerService>,
    /// List of targets that belong to this Load Balancer
    #[serde(rename = "targets")]
    pub targets: Vec<crate::models::Target>,
}

impl LoadBalancer {
    pub fn new(algorithm: crate::models::LoadBalancerAlgorithm, created: String, id: i32, included_traffic: i64, ingoing_traffic: Option<i64>, labels: ::std::collections::HashMap<String, String>, load_balancer_type: crate::models::LoadBalancerType, location: crate::models::Location, name: String, outgoing_traffic: Option<i64>, private_net: Vec<crate::models::LoadBalancerPrivateNet>, protection: crate::models::Protection, public_net: crate::models::LoadBalancerPublicNet, services: Vec<crate::models::LoadBalancerService>, targets: Vec<crate::models::Target>) -> LoadBalancer {
        LoadBalancer {
            algorithm: Box::new(algorithm),
            created,
            id,
            included_traffic,
            ingoing_traffic,
            labels,
            load_balancer_type: Box::new(load_balancer_type),
            location: Box::new(location),
            name,
            outgoing_traffic,
            private_net,
            protection: Box::new(protection),
            public_net: Box::new(public_net),
            services,
            targets,
        }
    }
}



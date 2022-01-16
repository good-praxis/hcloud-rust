/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateLoadBalancerRequest : Request for POST https://api.hetzner.cloud/v1/load_balancers



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateLoadBalancerRequest {
    #[serde(rename = "algorithm")]
    pub algorithm: Box<crate::models::LoadBalancerAlgorithm>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// ID or name of the Load Balancer type this Load Balancer should be created with
    #[serde(rename = "load_balancer_type")]
    pub load_balancer_type: String,
    /// ID or name of Location to create Load Balancer in
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Name of the Load Balancer
    #[serde(rename = "name")]
    pub name: String,
    /// ID of the network the Load Balancer should be attached to on creation
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<i32>,
    /// Name of network zone
    #[serde(rename = "network_zone", skip_serializing_if = "Option::is_none")]
    pub network_zone: Option<String>,
    /// Enable or disable the public interface of the Load Balancer
    #[serde(rename = "public_interface", skip_serializing_if = "Option::is_none")]
    pub public_interface: Option<bool>,
    /// Array of services
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::LoadBalancerService>>,
    /// Array of targets
    #[serde(rename = "targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<crate::models::Target>>,
}

impl CreateLoadBalancerRequest {
    /// Request for POST https://api.hetzner.cloud/v1/load_balancers
    pub fn new(algorithm: crate::models::LoadBalancerAlgorithm, load_balancer_type: String, name: String) -> CreateLoadBalancerRequest {
        CreateLoadBalancerRequest {
            algorithm: Box::new(algorithm),
            labels: None,
            load_balancer_type,
            location: None,
            name,
            network: None,
            network_zone: None,
            public_interface: None,
            services: None,
            targets: None,
        }
    }
}



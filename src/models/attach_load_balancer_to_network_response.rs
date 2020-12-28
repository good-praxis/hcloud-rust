/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttachLoadBalancerToNetworkResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachLoadBalancerToNetworkResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl AttachLoadBalancerToNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network
    pub fn new(action: crate::models::Action) -> AttachLoadBalancerToNetworkResponse {
        AttachLoadBalancerToNetworkResponse {
            action,
        }
    }
}



/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttachLoadBalancerToNetworkResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachLoadBalancerToNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AttachLoadBalancerToNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/attach_to_network
    pub fn new(action: crate::models::Action) -> AttachLoadBalancerToNetworkResponse {
        AttachLoadBalancerToNetworkResponse {
            action: Box::new(action),
        }
    }
}



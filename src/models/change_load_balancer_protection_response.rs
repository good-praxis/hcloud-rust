/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeLoadBalancerProtectionResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeLoadBalancerProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeLoadBalancerProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeLoadBalancerProtectionResponse {
        ChangeLoadBalancerProtectionResponse {
            action: Box::new(action),
        }
    }
}



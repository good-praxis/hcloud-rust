/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SetRulesRequest : Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/set_rules



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetRulesRequest {
    /// Array of rules
    #[serde(rename = "rules")]
    pub rules: Vec<crate::models::Rule>,
}

impl SetRulesRequest {
    /// Request for POST https://api.hetzner.cloud/v1/firewalls/{id}/actions/set_rules
    pub fn new(rules: Vec<crate::models::Rule>) -> SetRulesRequest {
        SetRulesRequest {
            rules,
        }
    }
}



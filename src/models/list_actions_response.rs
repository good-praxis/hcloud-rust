/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListActionsResponse : Response to GET https://api.hetzner.cloud/v1/actions



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListActionsResponse {
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::Action>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::models::Meta>,
}

impl ListActionsResponse {
    /// Response to GET https://api.hetzner.cloud/v1/actions
    pub fn new(actions: Vec<crate::models::Action>) -> ListActionsResponse {
        ListActionsResponse {
            actions,
            meta: None,
        }
    }
}



/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFloatingIpResponse : Response to GET https://api.hetzner.cloud/v1/floating_ips/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFloatingIpResponse {
    #[serde(rename = "floating_ip")]
    pub floating_ip: Box<crate::models::FloatingIp>,
}

impl GetFloatingIpResponse {
    /// Response to GET https://api.hetzner.cloud/v1/floating_ips/{id}
    pub fn new(floating_ip: crate::models::FloatingIp) -> GetFloatingIpResponse {
        GetFloatingIpResponse {
            floating_ip: Box::new(floating_ip),
        }
    }
}



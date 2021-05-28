/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeReverseDnsEntryForFloatingIpRequest : Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeReverseDnsEntryForFloatingIpRequest {
    /// IP address for which to set the reverse DNS entry
    #[serde(rename = "ip")]
    pub ip: String,
    /// Hostname to set as a reverse DNS PTR entry, will reset to original default value if `null`
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: Option<String>,
}

impl ChangeReverseDnsEntryForFloatingIpRequest {
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_dns_ptr
    pub fn new(ip: String, dns_ptr: Option<String>) -> ChangeReverseDnsEntryForFloatingIpRequest {
        ChangeReverseDnsEntryForFloatingIpRequest {
            ip,
            dns_ptr,
        }
    }
}



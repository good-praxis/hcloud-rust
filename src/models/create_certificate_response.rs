/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateCertificateResponse : Response to POST https://api.hetzner.cloud/v1/certificates



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCertificateResponse {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
    #[serde(rename = "certificate")]
    pub certificate: Box<crate::models::Certificate>,
}

impl CreateCertificateResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/certificates
    pub fn new(certificate: crate::models::Certificate) -> CreateCertificateResponse {
        CreateCertificateResponse {
            action: None,
            certificate: Box::new(certificate),
        }
    }
}



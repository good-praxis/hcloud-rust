/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPricesResponsePricing {
    /// Currency the returned prices are expressed in, coded according to ISO 4217
    #[serde(rename = "currency")]
    pub currency: String,
    /// The VAT rate used for calculating prices with VAT
    #[serde(rename = "vat_rate")]
    pub vat_rate: String,
    #[serde(rename = "image")]
    pub image: Box<crate::models::ListPricesResponsePricingImage>,
    #[serde(rename = "floating_ip")]
    pub floating_ip: Box<crate::models::ListPricesResponsePricingFloatingIp>,
    #[serde(rename = "traffic")]
    pub traffic: Box<crate::models::ListPricesResponsePricingTraffic>,
    #[serde(rename = "server_backup")]
    pub server_backup: Box<crate::models::ListPricesResponsePricingServerBackup>,
    #[serde(rename = "volume")]
    pub volume: Box<crate::models::ListPricesResponsePricingVolume>,
    /// Costs of Server types per Location and type
    #[serde(rename = "server_types")]
    pub server_types: Vec<crate::models::ListPricesResponsePricingServerTypes>,
    /// Costs of Load Balancer types per Location and type
    #[serde(rename = "load_balancer_types")]
    pub load_balancer_types: Vec<crate::models::ListPricesResponsePricingLoadBalancerTypes>,
}

impl ListPricesResponsePricing {
    pub fn new(currency: String, vat_rate: String, image: crate::models::ListPricesResponsePricingImage, floating_ip: crate::models::ListPricesResponsePricingFloatingIp, traffic: crate::models::ListPricesResponsePricingTraffic, server_backup: crate::models::ListPricesResponsePricingServerBackup, volume: crate::models::ListPricesResponsePricingVolume, server_types: Vec<crate::models::ListPricesResponsePricingServerTypes>, load_balancer_types: Vec<crate::models::ListPricesResponsePricingLoadBalancerTypes>) -> ListPricesResponsePricing {
        ListPricesResponsePricing {
            currency,
            vat_rate,
            image: Box::new(image),
            floating_ip: Box::new(floating_ip),
            traffic: Box::new(traffic),
            server_backup: Box::new(server_backup),
            volume: Box::new(volume),
            server_types,
            load_balancer_types,
        }
    }
}



/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListPricesResponsePricingVolume : The cost of Volume per GB/month



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPricesResponsePricingVolume {
    #[serde(rename = "price_per_gb_month")]
    pub price_per_gb_month: Box<crate::models::Price>,
}

impl ListPricesResponsePricingVolume {
    /// The cost of Volume per GB/month
    pub fn new(price_per_gb_month: crate::models::Price) -> ListPricesResponsePricingVolume {
        ListPricesResponsePricingVolume {
            price_per_gb_month: Box::new(price_per_gb_month),
        }
    }
}



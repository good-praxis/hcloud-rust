/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPricesResponsePricingServerTypes {
    /// ID of the Server type the price is for
    #[serde(rename = "id")]
    pub id: i32,
    /// Name of the Server type the price is for
    #[serde(rename = "name")]
    pub name: String,
    /// Server type costs per Location
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::PricePerTime>,
}

impl ListPricesResponsePricingServerTypes {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        name: String,
        prices: Vec<crate::models::PricePerTime>,
    ) -> ListPricesResponsePricingServerTypes {
        ListPricesResponsePricingServerTypes { id, name, prices }
    }
}

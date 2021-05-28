/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Location : Location the Floating IP was created in. Routing is optimized for this Location. | Location of the Volume. Volume can only be attached to Servers in the same Location.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// ID of the Location
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique identifier of the Location
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the Location
    #[serde(rename = "description")]
    pub description: String,
    /// ISO 3166-1 alpha-2 code of the country the Location resides in
    #[serde(rename = "country")]
    pub country: String,
    /// City the Location is closest to
    #[serde(rename = "city")]
    pub city: String,
    /// Latitude of the city closest to the Location
    #[serde(rename = "latitude")]
    pub latitude: f32,
    /// Longitude of the city closest to the Location
    #[serde(rename = "longitude")]
    pub longitude: f32,
    /// Name of network zone this Location resides in
    #[serde(rename = "network_zone")]
    pub network_zone: String,
}

impl Location {
    /// Location the Floating IP was created in. Routing is optimized for this Location. | Location of the Volume. Volume can only be attached to Servers in the same Location.
    pub fn new(id: i32, name: String, description: String, country: String, city: String, latitude: f32, longitude: f32, network_zone: String) -> Location {
        Location {
            id,
            name,
            description,
            country,
            city,
            latitude,
            longitude,
            network_zone,
        }
    }
}



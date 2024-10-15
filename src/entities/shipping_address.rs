use serde::{Deserialize, Serialize};

/// This object represents a shipping address.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#shippingaddress)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    /// Two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code
    pub country_code: String,

    /// State, if applicable
    pub state: String,

    /// City
    pub city: String,

    /// First line for the address
    pub street_line1: String,

    /// Second line for the address
    pub street_line2: String,

    /// Address post code
    pub post_code: String,
}

// Divider: all content below this line will be preserved after code regen

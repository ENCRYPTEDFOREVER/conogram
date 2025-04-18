use serde::{Deserialize, Serialize};

/// Describes the physical address of a location.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#locationaddress)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocationAddress {
    /// The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
    pub country_code: String,

    /// *Optional*. State of the location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// *Optional*. City of the location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// *Optional*. Street address of the location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
}

// Divider: all content below this line will be preserved after code regen

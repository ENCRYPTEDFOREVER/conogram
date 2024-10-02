use crate::entities::location::Location;
use serde::{Deserialize, Serialize};

/// Contains information about the location of a Telegram Business account.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#businesslocation)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BusinessLocation {
    /// Address of the business
    pub address: String,

    /// *Optional*. Location of the business
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

// Divider: all content below this line will be preserved after code regen

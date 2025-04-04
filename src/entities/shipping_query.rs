use serde::{Deserialize, Serialize};

use crate::entities::{shipping_address::ShippingAddress, user::User};

/// This object contains information about an incoming shipping query.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#shippingquery)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,

    /// User who sent the query
    pub from: User,

    /// Bot-specified invoice payload
    pub invoice_payload: String,

    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

// Divider: all content below this line will be preserved after code regen

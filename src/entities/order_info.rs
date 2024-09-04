use crate::entities::shipping_address::ShippingAddress;
use serde::{Deserialize, Serialize};

///This object represents information about an order.
///
///API Reference: [link](https://core.telegram.org/bots/api/#orderinfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderInfo {
    ///*Optional*. User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///*Optional*. User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    ///*Optional*. User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    ///*Optional*. User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}
// Divider: all content below this line will be preserved after code regen

use crate::entities::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};

///This object represents one shipping option.
///
///API Reference: [link](https://core.telegram.org/bots/api/#shippingoption)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ShippingOption {
    ///Shipping option identifier
    pub id: String,

    ///Option title
    pub title: String,

    ///List of price portions
    pub prices: Vec<LabeledPrice>,
}
// Divider: all content below this line will be preserved after code regen

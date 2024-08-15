use crate::entities::chat_boost_source_gift_code::ChatBoostSourceGiftCode;
use crate::entities::chat_boost_source_giveaway::ChatBoostSourceGiveaway;
use crate::entities::chat_boost_source_premium::ChatBoostSourcePremium;
use serde::{Deserialize, Serialize};

///This object describes the source of a chat boost. It can be one of
///
///* [ChatBoostSourcePremium](https://core.telegram.org/bots/api/#chatboostsourcepremium)
///* [ChatBoostSourceGiftCode](https://core.telegram.org/bots/api/#chatboostsourcegiftcode)
///* [ChatBoostSourceGiveaway](https://core.telegram.org/bots/api/#chatboostsourcegiveaway)
///
///API Reference: [link](https://core.telegram.org/bots/api/#chatboostsource)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum ChatBoostSource {
    #[serde(rename = "premium")]
    Premium(ChatBoostSourcePremium),
    #[serde(rename = "gift_code")]
    GiftCode(ChatBoostSourceGiftCode),
    #[serde(rename = "giveaway")]
    Giveaway(ChatBoostSourceGiveaway),
}
impl Default for ChatBoostSource {
    fn default() -> Self {
        Self::Premium(ChatBoostSourcePremium::default())
    }
}
impl From<ChatBoostSourcePremium> for ChatBoostSource {
    fn from(value: ChatBoostSourcePremium) -> Self {
        Self::Premium(value)
    }
}

impl From<ChatBoostSourceGiftCode> for ChatBoostSource {
    fn from(value: ChatBoostSourceGiftCode) -> Self {
        Self::GiftCode(value)
    }
}

impl From<ChatBoostSourceGiveaway> for ChatBoostSource {
    fn from(value: ChatBoostSourceGiveaway) -> Self {
        Self::Giveaway(value)
    }
}
// Divider: all content below this line will be preserved after code regen

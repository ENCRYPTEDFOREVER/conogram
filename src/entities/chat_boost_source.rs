use serde::{Deserialize, Serialize};

use crate::entities::{
    chat_boost_source_gift_code::ChatBoostSourceGiftCode,
    chat_boost_source_giveaway::ChatBoostSourceGiveaway,
    chat_boost_source_premium::ChatBoostSourcePremium,
};

/// This object describes the source of a chat boost. It can be one of
///
/// * [ChatBoostSourcePremium](https://core.telegram.org/bots/api/#chatboostsourcepremium)
/// * [ChatBoostSourceGiftCode](https://core.telegram.org/bots/api/#chatboostsourcegiftcode)
/// * [ChatBoostSourceGiveaway](https://core.telegram.org/bots/api/#chatboostsourcegiveaway)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatboostsource)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum ChatBoostSource {
    /// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcepremium)
    #[serde(rename = "premium")]
    Premium(ChatBoostSourcePremium),

    /// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcegiftcode)
    #[serde(rename = "gift_code")]
    GiftCode(ChatBoostSourceGiftCode),

    /// The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and *prize\_star\_count* / 500 times for one year for Telegram Star giveaways.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#chatboostsourcegiveaway)
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

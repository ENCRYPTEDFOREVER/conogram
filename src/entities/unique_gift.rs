use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        chat::Chat, unique_gift_backdrop::UniqueGiftBackdrop, unique_gift_colors::UniqueGiftColors,
        unique_gift_model::UniqueGiftModel, unique_gift_symbol::UniqueGiftSymbol,
    },
    utils::deserialize_utils::is_false,
};

/// This object describes a unique gift that was upgraded from a regular gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegift)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UniqueGift {
    /// Identifier of the regular gift from which the gift was upgraded
    pub gift_id: String,

    /// Human-readable name of the regular gift from which this unique gift was upgraded
    pub base_name: String,

    /// Unique name of the gift. This name can be used in `https://t.me/nft/...` links and story areas.
    pub name: String,

    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift
    pub number: i64,

    /// Model of the gift
    pub model: UniqueGiftModel,

    /// Symbol of the gift
    pub symbol: UniqueGiftSymbol,

    /// Backdrop of the gift
    pub backdrop: UniqueGiftBackdrop,

    /// *Optional*. *True*, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_premium: bool,

    /// *Optional*. *True*, if the gift was used to craft another gift and isn't available anymore
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_burned: bool,

    /// *Optional*. *True*, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_from_blockchain: bool,

    /// *Optional*. The color scheme that can be used by the gift's owner for the chat's name, replies to messages and link previews; for business account gifts and gifts that are currently on sale only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colors: Option<UniqueGiftColors>,

    /// *Optional*. Information about the chat that published the gift
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Box<Chat>>,
}

// Divider: all content below this line will be preserved after code regen

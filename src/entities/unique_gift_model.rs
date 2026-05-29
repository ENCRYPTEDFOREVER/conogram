use serde::{Deserialize, Serialize};

use crate::entities::sticker::Sticker;

/// This object describes the model of a unique gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegiftmodel)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftModel {
    /// Name of the model
    pub name: String,

    /// The sticker that represents the unique gift
    pub sticker: Sticker,

    /// The number of unique gifts that receive this model for every 1000 gift upgrades. Always 0 for crafted gifts.
    pub rarity_per_mille: i64,

    /// *Optional*. Rarity of the model if it is a crafted model. Currently, can be “uncommon”, “rare”, “epic”, or “legendary”.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rarity: Option<UniqueGiftModelRarity>,
}

/// *Optional*. Rarity of the model if it is a crafted model. Currently, can be “uncommon”, “rare”, “epic”, or “legendary”.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum UniqueGiftModelRarity {
    /// `uncommon`
    #[default]
    #[serde(rename = "uncommon")]
    Uncommon,

    /// `rare`
    #[serde(rename = "rare")]
    Rare,

    /// `epic`
    #[serde(rename = "epic")]
    Epic,

    /// `legendary`
    #[serde(rename = "legendary")]
    Legendary,
}

// Divider: all content below this line will be preserved after code regen

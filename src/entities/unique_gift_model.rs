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

    /// The number of unique gifts that receive this model for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}

// Divider: all content below this line will be preserved after code regen

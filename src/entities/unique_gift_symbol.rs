use serde::{Deserialize, Serialize};

use crate::entities::sticker::Sticker;

/// This object describes the symbol shown on the pattern of a unique gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegiftsymbol)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol
    pub name: String,

    /// The sticker that represents the unique gift
    pub sticker: Sticker,

    /// The number of unique gifts that receive this model for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}

// Divider: all content below this line will be preserved after code regen

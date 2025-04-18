use serde::{Deserialize, Serialize};

use crate::entities::unique_gift_backdrop_colors::UniqueGiftBackdropColors;

/// This object describes the backdrop of a unique gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegiftbackdrop)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop
    pub name: String,

    /// Colors of the backdrop
    pub colors: UniqueGiftBackdropColors,

    /// The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}

// Divider: all content below this line will be preserved after code regen

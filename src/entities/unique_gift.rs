use serde::{Deserialize, Serialize};

use crate::entities::{
    unique_gift_backdrop::UniqueGiftBackdrop, unique_gift_model::UniqueGiftModel,
    unique_gift_symbol::UniqueGiftSymbol,
};

/// This object describes a unique gift that was upgraded from a regular gift.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#uniquegift)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UniqueGift {
    /// Human-readable name of the regular gift from which this unique gift was upgraded
    pub base_name: String,

    /// Unique name of the gift. This name can be used in `https://t.me/nft/...` links and story areas
    pub name: String,

    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift
    pub number: i64,

    /// Model of the gift
    pub model: UniqueGiftModel,

    /// Symbol of the gift
    pub symbol: UniqueGiftSymbol,

    /// Backdrop of the gift
    pub backdrop: UniqueGiftBackdrop,
}

// Divider: all content below this line will be preserved after code regen

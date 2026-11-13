use conogram_derives::Request;
use serde::Serialize;

use crate::entities::unique_gift::UniqueGift;

/// Use this method to get information about a unique gift by its name. Returns a [UniqueGift](https://core.telegram.org/bots/api/#uniquegift) object on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getuniquegift)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = UniqueGift)]
pub struct GetUniqueGiftParams {
    /// Unique name of the gift
    pub name: String,
}

// Divider: all content below this line will be preserved after code regen

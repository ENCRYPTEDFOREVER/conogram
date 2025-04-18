use serde::{Deserialize, Serialize};

/// This object describes the types of gifts that can be gifted to a user or a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#acceptedgifttypes)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptedGiftTypes {
    /// True, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,

    /// True, if limited regular gifts are accepted
    pub limited_gifts: bool,

    /// True, if unique gifts or gifts that can be upgraded to unique for free are accepted
    pub unique_gifts: bool,

    /// True, if a Telegram Premium subscription is accepted
    pub premium_subscription: bool,
}

// Divider: all content below this line will be preserved after code regen

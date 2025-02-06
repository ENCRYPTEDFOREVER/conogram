use serde::{Deserialize, Serialize};

use crate::entities::{
    reaction_type_custom_emoji::ReactionTypeCustomEmoji, reaction_type_emoji::ReactionTypeEmoji,
    reaction_type_paid::ReactionTypePaid,
};

/// This object describes the type of a reaction. Currently, it can be one of
///
/// * [ReactionTypeEmoji](https://core.telegram.org/bots/api/#reactiontypeemoji)
/// * [ReactionTypeCustomEmoji](https://core.telegram.org/bots/api/#reactiontypecustomemoji)
/// * [ReactionTypePaid](https://core.telegram.org/bots/api/#reactiontypepaid)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#reactiontype)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReactionType {
    /// The reaction is based on an emoji.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#reactiontypeemoji)
    #[serde(rename = "emoji")]
    Emoji(ReactionTypeEmoji),

    /// The reaction is based on a custom emoji.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#reactiontypecustomemoji)
    #[serde(rename = "custom_emoji")]
    CustomEmoji(ReactionTypeCustomEmoji),

    /// The reaction is paid.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#reactiontypepaid)
    #[serde(rename = "paid")]
    Paid(ReactionTypePaid),
}

impl Default for ReactionType {
    fn default() -> Self {
        Self::Emoji(ReactionTypeEmoji::default())
    }
}

impl From<ReactionTypeEmoji> for ReactionType {
    fn from(value: ReactionTypeEmoji) -> Self {
        Self::Emoji(value)
    }
}

impl From<ReactionTypeCustomEmoji> for ReactionType {
    fn from(value: ReactionTypeCustomEmoji) -> Self {
        Self::CustomEmoji(value)
    }
}

impl From<ReactionTypePaid> for ReactionType {
    fn from(value: ReactionTypePaid) -> Self {
        Self::Paid(value)
    }
}

// Divider: all content below this line will be preserved after code regen
use super::reaction_type_emoji::ReactionEmoji;

impl From<ReactionEmoji> for ReactionType {
    fn from(emoji: ReactionEmoji) -> Self {
        Self::Emoji(ReactionTypeEmoji { emoji })
    }
}

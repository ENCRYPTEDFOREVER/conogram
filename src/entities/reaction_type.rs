use crate::entities::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
use crate::entities::reaction_type_emoji::ReactionTypeEmoji;
use serde::{Deserialize, Serialize};

///This object describes the type of a reaction. Currently, it can be one of
///
///* [ReactionTypeEmoji](https://core.telegram.org/bots/api/#reactiontypeemoji)
///* [ReactionTypeCustomEmoji](https://core.telegram.org/bots/api/#reactiontypecustomemoji)
///API Reference: [link](https://core.telegram.org/bots/api/#reactiontype)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReactionType {
    #[serde(rename = "emoji")]
    Emoji(ReactionTypeEmoji),
    #[serde(rename = "custom_emoji")]
    CustomEmoji(ReactionTypeCustomEmoji),
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
// Divider: all content below this line will be preserved after code regen

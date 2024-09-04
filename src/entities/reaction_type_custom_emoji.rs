use serde::{Deserialize, Serialize};

///The reaction is based on a custom emoji.
///
///API Reference: [link](https://core.telegram.org/bots/api/#reactiontypecustomemoji)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ReactionTypeCustomEmoji {
    ///Custom emoji identifier
    pub custom_emoji_id: String,
}
// Divider: all content below this line will be preserved after code regen

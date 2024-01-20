use crate::entities::chat::Chat;
use crate::entities::reaction_count::ReactionCount;
use crate::utils::deserialize_utils::deserialize_boxed;
use serde::{Deserialize, Serialize};

///This object represents reaction changes on a message with anonymous reactions.
///API Reference: [link](https://core.telegram.org/bots/api/#messagereactioncountupdated)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    ///The chat containing the message
    #[serde(deserialize_with = "deserialize_boxed")]
    pub chat: Box<Chat>,

    ///Unique message identifier inside the chat
    pub message_id: i64,

    ///Date of the change in Unix time
    pub date: i64,

    ///List of reactions that are present on the message
    pub reactions: Vec<ReactionCount>,
}
// Divider: all content below this line will be preserved after code regen

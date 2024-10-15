use serde::{Deserialize, Serialize};

use crate::entities::{chat::Chat, reaction_type::ReactionType, user::User};

/// This object represents a change of a reaction on a message performed by a user.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messagereactionupdated)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Box<Chat>,

    /// Unique identifier of the message inside the chat
    pub message_id: i64,

    /// *Optional*. The user that changed the reaction, if the user isn't anonymous
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// *Optional*. The chat on behalf of which the reaction was changed, if the user is anonymous
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Box<Chat>>,

    /// Date of the change in Unix time
    pub date: i64,

    /// Previous list of reaction types that were set by the user
    pub old_reaction: Vec<ReactionType>,

    /// New list of reaction types that have been set by the user
    pub new_reaction: Vec<ReactionType>,
}

// Divider: all content below this line will be preserved after code regen

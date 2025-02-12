use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{misc::chat_id::ChatId, reaction_type::ReactionType},
    utils::deserialize_utils::is_false,
};

/// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmessagereaction)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetMessageReactionParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    pub message_id: i64,

    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<ReactionType>,

    /// Pass *True* to set the reaction with a big animation
    #[serde(skip_serializing_if = "is_false")]
    pub is_big: bool,
}

// Divider: all content below this line will be preserved after code regen

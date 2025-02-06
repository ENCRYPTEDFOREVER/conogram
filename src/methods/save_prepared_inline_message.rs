use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{
        inline_query_result::InlineQueryResult, prepared_inline_message::PreparedInlineMessage,
    },
    utils::deserialize_utils::is_false,
};

/// Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#savepreparedinlinemessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = PreparedInlineMessage)]
pub struct SavePreparedInlineMessageParams {
    /// Unique identifier of the target user that can use the prepared message
    pub user_id: i64,

    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,

    /// Pass *True* if the message can be sent to private chats with users
    #[serde(skip_serializing_if = "is_false")]
    pub allow_user_chats: bool,

    /// Pass *True* if the message can be sent to private chats with bots
    #[serde(skip_serializing_if = "is_false")]
    pub allow_bot_chats: bool,

    /// Pass *True* if the message can be sent to group and supergroup chats
    #[serde(skip_serializing_if = "is_false")]
    pub allow_group_chats: bool,

    /// Pass *True* if the message can be sent to channel chats
    #[serde(skip_serializing_if = "is_false")]
    pub allow_channel_chats: bool,
}

// Divider: all content below this line will be preserved after code regen

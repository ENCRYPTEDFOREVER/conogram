use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{message_id::MessageId, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#forwardmessages)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<MessageId>)]
pub struct ForwardMessagesParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)
    pub from_chat_id: ChatId,

    /// A JSON-serialized list of 1-100 identifiers of messages in the chat *from\_chat\_id* to forward. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<i64>,

    /// Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the forwarded messages from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,
}

// Divider: all content below this line will be preserved after code regen

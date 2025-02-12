use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{message::Message, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#forwardmessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Message)]
pub struct ForwardMessageParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
    pub from_chat_id: ChatId,

    /// New start timestamp for the forwarded video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_timestamp: Option<i64>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the forwarded message from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,

    /// Message identifier in the chat specified in *from\_chat\_id*
    pub message_id: i64,
}

// Divider: all content below this line will be preserved after code regen

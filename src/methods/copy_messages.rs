use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{message_id::MessageId, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\_option\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#copymessages)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Vec<MessageId>)]
pub struct CopyMessagesParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)
    pub from_chat_id: ChatId,

    /// A JSON-serialized list of 1-100 identifiers of messages in the chat *from\_chat\_id* to copy. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<i64>,

    /// Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,

    /// Pass *True* to copy the messages without their captions
    #[serde(skip_serializing_if = "is_false")]
    pub remove_caption: bool,
}

// Divider: all content below this line will be preserved after code regen

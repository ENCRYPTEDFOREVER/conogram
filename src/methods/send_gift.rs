use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{message_entity::MessageEntity, misc::chat_id::ChatId},
    utils::deserialize_utils::is_false,
};

/// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendgift)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SendGiftParams {
    /// Required if *chat\_id* is not specified. Unique identifier of the target user who will receive the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Required if *user\_id* is not specified. Unique identifier for the chat or username of the channel (in the format `@channelusername`) that will receive the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,

    /// Identifier of the gift
    pub gift_id: String,

    /// Pass *True* to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
    #[serde(skip_serializing_if = "is_false")]
    pub pay_for_upgrade: bool,

    /// Text that will be shown along with the gift; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\_parse\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,
}

// Divider: all content below this line will be preserved after code regen

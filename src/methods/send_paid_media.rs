use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{
        input_paid_media::InputPaidMedia,
        message::Message,
        message_entity::MessageEntity,
        misc::{chat_id::ChatId, reply_markup::ReplyMarkup},
        reply_parameters::ReplyParameters,
    },
    utils::deserialize_utils::is_false,
};

/// Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendpaidmedia)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Message)]
pub struct SendPaidMediaParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`). If the chat is a channel, all Telegram Star proceeds from this media will be credited to the chat's balance. Otherwise, they will be credited to the bot's balance.
    pub chat_id: ChatId,

    /// The number of Telegram Stars that must be paid to buy access to the media; 1-10000
    pub star_count: i64,

    /// A JSON-serialized array describing the media to be sent; up to 10 items
    pub media: Vec<InputPaidMedia>,

    /// Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    /// Media caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the media caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// Pass *True*, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,

    /// Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "is_false")]
    pub allow_paid_broadcast: bool,

    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

// Divider: all content below this line will be preserved after code regen

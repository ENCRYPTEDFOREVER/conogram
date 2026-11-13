use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{
        message::Message,
        message_entity::MessageEntity,
        misc::{chat_id::ChatId, message_effects::MessageEffect, reply_markup::ReplyMarkup},
        reply_parameters::ReplyParameters,
    },
    utils::deserialize_utils::is_false,
};

/// Use this method to send a message with an [HTML widget](https://core.telegram.org/bots/widgets). Currently, in the production environment, widgets can be sent only to private chats, and sent widgets automatically have protected content and therefore can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendwidget)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Message)]
pub struct SendWidgetParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// HTML code of the widget, 1-32768 bytes. The code must not exceed 4096 bytes when compressed by zlib with the maximum compression level.
    pub html: String,

    /// Width of the widget in logical pixels; omit if the widget must occupy the whole width of the message bubble. The width is clamped by the app to the range from 200 to the maximum width of the message bubble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// Height of the widget in logical pixels; omit if the height of the widget isn't fixed. The height is clamped by the app to the range 50-600.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// Public data of the widget, 0-1024 bytes, which will be available wherever the widget is shown. If the widget is shared, then all messages with it will have the same public data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,

    /// Data of the widget, which is specific to the message, 0-1024 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data: Option<String>,

    /// Widget caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Mode for parsing entities in the widget caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// Pass *True* if the caption must be shown above the widget
    #[serde(skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// Pass *True* to allow the widget to send queries to the bot
    #[serde(skip_serializing_if = "is_false")]
    pub allow_queries: bool,

    /// Pass *True* to create a shared widget, which can be sent again using the method [sendSharedWidget](https://core.telegram.org/bots/api/#sendsharedwidget)
    #[serde(skip_serializing_if = "is_false")]
    pub is_shared: bool,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,

    /// Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[serde(skip_serializing_if = "is_false")]
    pub allow_paid_broadcast: bool,

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<MessageEffect>,

    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

// Divider: all content below this line will be preserved after code regen

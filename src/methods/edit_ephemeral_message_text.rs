use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_keyboard_markup::InlineKeyboardMarkup, link_preview_options::LinkPreviewOptions,
    message_entity::MessageEntity, misc::chat_id::ChatId,
};

/// Use this method to edit an ephemeral text message. Note that it is not guaranteed that the user will receive the message edit event, especially if they are offline. On success, *True* is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editephemeralmessagetext)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct EditEphemeralMessageTextParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Identifier of the user who received the message
    pub receiver_user_id: i64,

    /// Identifier of the ephemeral message to edit
    pub ephemeral_message_id: i64,

    /// New text of the message, 1-4096 characters after entity parsing
    pub text: String,

    /// Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen

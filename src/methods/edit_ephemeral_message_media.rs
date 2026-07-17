use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_keyboard_markup::InlineKeyboardMarkup, input_media::InputMedia, misc::chat_id::ChatId,
};

/// Use this method to edit the media of an ephemeral message. Note that it is not guaranteed that the user will receive the message edit event, especially if they are offline. On success, *True* is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editephemeralmessagemedia)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct EditEphemeralMessageMediaParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Identifier of the user who received the message
    pub receiver_user_id: i64,

    /// Identifier of the ephemeral message to edit
    pub ephemeral_message_id: i64,

    /// A JSON-serialized object for the new media content of the message. A new file can't be uploaded; use a previously uploaded file via its file\_id or specify a URL.
    pub media: InputMedia,

    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen

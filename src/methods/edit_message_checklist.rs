use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{
    inline_keyboard_markup::InlineKeyboardMarkup, input_checklist::InputChecklist,
    message::Message, misc::chat_id::ChatId,
};

/// Use this method to edit a checklist on behalf of a connected business account. On success, the edited [Message](https://core.telegram.org/bots/api/#message) is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#editmessagechecklist)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Message)]
pub struct EditMessageChecklistParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: String,

    /// Unique identifier for the target chat or username of the target bot in the format `@username`
    pub chat_id: ChatId,

    /// Unique identifier for the target message
    pub message_id: i64,

    /// A JSON-serialized object for the new checklist
    pub checklist: InputChecklist,

    /// A JSON-serialized object for the new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen

use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{
        inline_keyboard_markup::InlineKeyboardMarkup,
        input_checklist::InputChecklist,
        message::Message,
        misc::{chat_id::ChatId, message_effects::MessageEffect},
        reply_parameters::ReplyParameters,
    },
    utils::deserialize_utils::is_false,
};

/// Use this method to send a checklist on behalf of a connected business account. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendchecklist)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Message)]
pub struct SendChecklistParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: String,

    /// Unique identifier for the target chat or username of the target bot in the format `@username`
    pub chat_id: ChatId,

    /// A JSON-serialized object for the checklist to send
    pub checklist: InputChecklist,

    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,

    /// Unique identifier of the message effect to be added to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<MessageEffect>,

    /// A JSON-serialized object for description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen
